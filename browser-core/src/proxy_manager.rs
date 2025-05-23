use crate::ProxyConfig;
use anyhow::{Result, anyhow};
use parking_lot::RwLock;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use reqwest::Client;
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct ProxyHealth {
    pub last_check: Instant,
    pub is_healthy: bool,
    pub response_time: Duration,
    pub error_count: u32,
}

pub struct ProxyManager {
    proxies: Arc<RwLock<Vec<ProxyConfig>>>,
    proxy_health: Arc<RwLock<HashMap<usize, ProxyHealth>>>,
    current_index: Arc<RwLock<usize>>,
}

impl ProxyManager {
    pub fn new() -> Self {
        Self {
            proxies: Arc::new(RwLock::new(Vec::new())),
            proxy_health: Arc::new(RwLock::new(HashMap::new())),
            current_index: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn load_proxies(&self, proxies: Vec<ProxyConfig>) -> Result<()> {
        {
            let mut proxy_list = self.proxies.write();
            *proxy_list = proxies;
        }

        // Initialize health status for all proxies
        {
            let mut health_map = self.proxy_health.write();
            health_map.clear();
            let proxy_count = self.proxies.read().len();
            for i in 0..proxy_count {
                health_map.insert(i, ProxyHealth {
                    last_check: Instant::now(),
                    is_healthy: true, // Assume healthy initially
                    response_time: Duration::from_millis(0),
                    error_count: 0,
                });
            }
        }

        // Start health checking
        self.start_health_checking().await;
        
        Ok(())
    }

    pub async fn get_proxy(&self, index: usize) -> Result<Option<String>> {
        let proxies = self.proxies.read();
        
        if index >= proxies.len() {
            return Err(anyhow!("Proxy index {} out of bounds", index));
        }

        let proxy = &proxies[index];
        let proxy_url = self.build_proxy_url(proxy)?;
        
        Ok(Some(proxy_url))
    }

    pub async fn get_next_healthy_proxy(&self) -> Result<Option<String>> {
        let proxies = self.proxies.read();
        let health_map = self.proxy_health.read();
        
        if proxies.is_empty() {
            return Ok(None);
        }

        // Find next healthy proxy
        let start_index = *self.current_index.read();
        let mut checked_count = 0;
        let mut current_idx = start_index;

        while checked_count < proxies.len() {
            if let Some(health) = health_map.get(&current_idx) {
                if health.is_healthy {
                    // Update current index
                    {
                        let mut current_index = self.current_index.write();
                        *current_index = (current_idx + 1) % proxies.len();
                    }
                    
                    let proxy = &proxies[current_idx];
                    let proxy_url = self.build_proxy_url(proxy)?;
                    return Ok(Some(proxy_url));
                }
            }
            
            current_idx = (current_idx + 1) % proxies.len();
            checked_count += 1;
        }

        // No healthy proxies found
        Ok(None)
    }

    fn build_proxy_url(&self, proxy: &ProxyConfig) -> Result<String> {
        match (&proxy.username, &proxy.password) {
            (Some(username), Some(password)) => {
                Ok(format!("{}://{}:{}@{}:{}", 
                    proxy.protocol, username, password, proxy.host, proxy.port))
            }
            _ => {
                Ok(format!("{}://{}:{}", 
                    proxy.protocol, proxy.host, proxy.port))
            }
        }
    }

    async fn start_health_checking(&self) {
        let proxies = self.proxies.clone();
        let proxy_health = self.proxy_health.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Check every 5 minutes
            
            loop {
                interval.tick().await;
                
                let proxy_list = proxies.read().clone();
                
                for (index, proxy) in proxy_list.iter().enumerate() {
                    if let Ok(proxy_url) = Self::build_proxy_url_static(proxy) {
                        let health_result = Self::check_proxy_health(&proxy_url).await;
                        
                        let mut health_map = proxy_health.write();
                        if let Some(health) = health_map.get_mut(&index) {
                            health.last_check = Instant::now();
                            match health_result {
                                Ok(response_time) => {
                                    health.is_healthy = true;
                                    health.response_time = response_time;
                                    health.error_count = 0;
                                }
                                Err(_) => {
                                    health.error_count += 1;
                                    // Mark as unhealthy after 3 consecutive failures
                                    if health.error_count >= 3 {
                                        health.is_healthy = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    fn build_proxy_url_static(proxy: &ProxyConfig) -> Result<String> {
        match (&proxy.username, &proxy.password) {
            (Some(username), Some(password)) => {
                Ok(format!("{}://{}:{}@{}:{}", 
                    proxy.protocol, username, password, proxy.host, proxy.port))
            }
            _ => {
                Ok(format!("{}://{}:{}", 
                    proxy.protocol, proxy.host, proxy.port))
            }
        }
    }

    async fn check_proxy_health(proxy_url: &str) -> Result<Duration> {
        let start_time = Instant::now();
        
        let proxy = reqwest::Proxy::all(proxy_url)
            .map_err(|e| anyhow!("Invalid proxy URL: {}", e))?;
        
        let client = Client::builder()
            .proxy(proxy)
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        // Test with a simple HTTP request
        let response = timeout(
            Duration::from_secs(15),
            client.get("http://httpbin.org/ip").send()
        ).await
        .map_err(|_| anyhow!("Request timeout"))?
        .map_err(|e| anyhow!("Request failed: {}", e))?;

        if response.status().is_success() {
            Ok(start_time.elapsed())
        } else {
            Err(anyhow!("HTTP request failed with status: {}", response.status()))
        }
    }

    pub async fn get_proxy_health_stats(&self) -> HashMap<usize, ProxyHealth> {
        self.proxy_health.read().clone()
    }

    pub async fn mark_proxy_unhealthy(&self, index: usize) {
        let mut health_map = self.proxy_health.write();
        if let Some(health) = health_map.get_mut(&index) {
            health.is_healthy = false;
            health.error_count += 1;
        }
    }

    pub async fn get_proxy_count(&self) -> usize {
        self.proxies.read().len()
    }
} 