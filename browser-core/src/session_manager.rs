use crate::AccountCredentials;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub id: String,
    pub username: String,
    pub email: String,
    pub cookies: String,
    pub fingerprint: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub status: String,
    pub activity_log: Vec<ActivityLog>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLog {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub details: Option<String>,
}

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_session(&self, session_id: &str, account: &AccountCredentials) -> Result<()> {
        let session_data = SessionData {
            id: session_id.to_string(),
            username: account.username.clone(),
            email: account.email.clone(),
            cookies: String::new(),
            fingerprint: String::new(),
            created_at: Utc::now(),
            last_activity: Utc::now(),
            status: "created".to_string(),
            activity_log: vec![ActivityLog {
                timestamp: Utc::now(),
                action: "session_created".to_string(),
                details: Some(format!("Created session for user: {}", account.username)),
            }],
        };

        self.sessions.write().insert(session_id.to_string(), session_data);
        
        Ok(())
    }

    pub async fn get_session(&self, session_id: &str) -> Option<SessionData> {
        self.sessions.read().get(session_id).cloned()
    }

    pub async fn update_session_cookies(&self, session_id: &str, cookies: &str) -> Result<()> {
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(session_id) {
            session.cookies = cookies.to_string();
            session.last_activity = Utc::now();
            session.activity_log.push(ActivityLog {
                timestamp: Utc::now(),
                action: "cookies_updated".to_string(),
                details: Some("Session cookies updated".to_string()),
            });
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn update_session_status(&self, session_id: &str, status: &str) -> Result<()> {
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(session_id) {
            let old_status = session.status.clone();
            session.status = status.to_string();
            session.last_activity = Utc::now();
            session.activity_log.push(ActivityLog {
                timestamp: Utc::now(),
                action: "status_changed".to_string(),
                details: Some(format!("Status changed from '{}' to '{}'", old_status, status)),
            });
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn log_activity(&self, session_id: &str, action: &str, details: Option<&str>) -> Result<()> {
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = Utc::now();
            session.activity_log.push(ActivityLog {
                timestamp: Utc::now(),
                action: action.to_string(),
                details: details.map(|s| s.to_string()),
            });
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write();
        if sessions.remove(session_id).is_some() {
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn get_all_sessions(&self) -> Vec<SessionData> {
        self.sessions.read().values().cloned().collect()
    }

    pub async fn get_sessions_by_status(&self, status: &str) -> Vec<SessionData> {
        self.sessions
            .read()
            .values()
            .filter(|session| session.status == status)
            .cloned()
            .collect()
    }

    pub async fn cleanup_inactive_sessions(&self, max_age_hours: u64) -> Result<usize> {
        let cutoff_time = Utc::now() - chrono::Duration::hours(max_age_hours as i64);
        let mut sessions = self.sessions.write();
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| session.last_activity > cutoff_time);
        
        let removed_count = initial_count - sessions.len();
        Ok(removed_count)
    }

    pub async fn get_session_count(&self) -> usize {
        self.sessions.read().len()
    }

    pub async fn is_session_active(&self, session_id: &str) -> bool {
        self.sessions
            .read()
            .get(session_id)
            .map(|session| {
                matches!(session.status.as_str(), "active" | "logged_in" | "warming_up")
            })
            .unwrap_or(false)
    }

    pub async fn get_session_activity_summary(&self, session_id: &str) -> Result<HashMap<String, u32>> {
        let sessions = self.sessions.read();
        if let Some(session) = sessions.get(session_id) {
            let mut summary = HashMap::new();
            for log in &session.activity_log {
                *summary.entry(log.action.clone()).or_insert(0) += 1;
            }
            Ok(summary)
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn update_session_fingerprint(&self, session_id: &str, fingerprint: &str) -> Result<()> {
        let mut sessions = self.sessions.write();
        if let Some(session) = sessions.get_mut(session_id) {
            session.fingerprint = fingerprint.to_string();
            session.last_activity = Utc::now();
            session.activity_log.push(ActivityLog {
                timestamp: Utc::now(),
                action: "fingerprint_updated".to_string(),
                details: Some("Browser fingerprint updated".to_string()),
            });
            Ok(())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn get_recent_activity(&self, session_id: &str, limit: usize) -> Result<Vec<ActivityLog>> {
        let sessions = self.sessions.read();
        if let Some(session) = sessions.get(session_id) {
            let mut logs = session.activity_log.clone();
            logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            logs.truncate(limit);
            Ok(logs)
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }
} 