// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{command, State, WebviewWindow, Manager};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::Connection;

mod anti_detection;
mod proxy_manager;
mod session_manager;
mod browser_engine;

use anti_detection::AntiDetectionEngine;
use proxy_manager::ProxyManager;
use session_manager::SessionManager;
use browser_engine::BrowserEngine;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountCredentials {
    pub username: String,
    pub password: String,
    pub email: String,
    pub proxy_index: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub id: String,
    pub account: AccountCredentials,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub status: String,
    pub proxy_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub protocol: String,
}

pub struct AppState {
    pub sessions: Arc<Mutex<HashMap<String, SessionInfo>>>,
    pub anti_detection: Arc<AntiDetectionEngine>,
    pub proxy_manager: Arc<ProxyManager>,
    pub session_manager: Arc<SessionManager>,
    pub browser_engine: Arc<BrowserEngine>,
}

#[command]
async fn create_session(
    account: AccountCredentials,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let session_id = Uuid::new_v4().to_string();
    
    // Assign proxy if available
    let proxy_url = if let Some(proxy_index) = account.proxy_index {
        state.proxy_manager.get_proxy(proxy_index).await
            .map_err(|e| format!("Proxy error: {}", e))?
    } else {
        None
    };
    
    let session_info = SessionInfo {
        id: session_id.clone(),
        account: account.clone(),
        created_at: Utc::now(),
        last_activity: Utc::now(),
        status: "initializing".to_string(),
        proxy_url,
    };
    
    state.sessions.lock().insert(session_id.clone(), session_info);
    
    // Initialize session with anti-detection
    state.session_manager.create_session(&session_id, &account).await
        .map_err(|e| format!("Session creation failed: {}", e))?;
    
    Ok(session_id)
}

#[command]
async fn login_account(
    session_id: String,
    state: State<'_, AppState>,
    window: WebviewWindow,
) -> Result<bool, String> {
    let session_info = {
        let sessions = state.sessions.lock();
        sessions.get(&session_id).cloned()
            .ok_or("Session not found")?
    };
    
    // Apply anti-detection measures
    state.anti_detection.setup_browser_fingerprint(&window).await
        .map_err(|e| format!("Anti-detection setup failed: {}", e))?;
    
    // Attempt login with human-like behavior
    state.browser_engine.login_with_behavior(&session_info, &window).await
        .map_err(|e| format!("Login failed: {}", e))
}

#[command]
async fn get_sessions(state: State<'_, AppState>) -> Result<Vec<SessionInfo>, String> {
    let sessions = state.sessions.lock();
    Ok(sessions.values().cloned().collect())
}

#[command]
async fn delete_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state.sessions.lock().remove(&session_id);
    state.session_manager.delete_session(&session_id).await
        .map_err(|e| format!("Session deletion failed: {}", e))?;
    Ok(true)
}

#[command]
async fn update_session_status(
    session_id: String,
    status: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let mut sessions = state.sessions.lock();
    if let Some(session) = sessions.get_mut(&session_id) {
        session.status = status;
        session.last_activity = Utc::now();
        Ok(true)
    } else {
        Err("Session not found".to_string())
    }
}

#[command]
async fn load_proxies(
    proxies: Vec<ProxyConfig>,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state.proxy_manager.load_proxies(proxies).await
        .map_err(|e| format!("Failed to load proxies: {}", e))?;
    Ok(true)
}

#[command]
async fn verify_2fa_code(
    session_id: String,
    code: String,
    state: State<'_, AppState>,
    window: WebviewWindow,
) -> Result<bool, String> {
    state.browser_engine.submit_2fa_code(&session_id, &code, &window).await
        .map_err(|e| format!("2FA verification failed: {}", e))
}

fn main() {
    tracing_subscriber::fmt::init();
    
    let anti_detection = Arc::new(AntiDetectionEngine::new());
    let proxy_manager = Arc::new(ProxyManager::new());
    let session_manager = Arc::new(SessionManager::new());
    let browser_engine = Arc::new(BrowserEngine::new());
    
    let app_state = AppState {
        sessions: Arc::new(Mutex::new(HashMap::new())),
        anti_detection,
        proxy_manager,
        session_manager,
        browser_engine,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            create_session,
            login_account,
            get_sessions,
            delete_session,
            update_session_status,
            load_proxies,
            verify_2fa_code
        ])
        .setup(|app| {
            // Initialize the database and other setup tasks
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = initialize_database(&app_handle).await {
                    eprintln!("Database initialization failed: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_database(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    
    let db_path = app_dir.join("instagram_automation.db");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    
    println!("Creating database at: {}", db_path.display());
    
    // Create database tables with proper connection string
    let mut connection = sqlx::sqlite::SqliteConnection::connect(&db_url).await?;
    
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_activity TEXT NOT NULL,
            status TEXT NOT NULL,
            proxy_url TEXT,
            cookies TEXT,
            fingerprint TEXT
        )
    "#).execute(&mut connection).await?;
    
    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS activity_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            action_type TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            details TEXT,
            FOREIGN KEY (session_id) REFERENCES sessions (id)
        )
    "#).execute(&mut connection).await?;
    
    Ok(())
}
