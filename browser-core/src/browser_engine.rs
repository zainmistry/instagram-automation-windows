use crate::SessionInfo;
use anyhow::Result;
use tauri::WebviewWindow;
use rand::Rng;
use tokio::time::{sleep, Duration};

pub struct BrowserEngine;

impl BrowserEngine {
    pub fn new() -> Self {
        Self
    }

    pub async fn login_with_behavior(&self, session_info: &SessionInfo, window: &WebviewWindow) -> Result<bool> {
        // Navigate to Instagram login page
        window.eval("window.location.href = 'https://www.instagram.com/accounts/login/'")?;
        
        // Wait for page to load
        sleep(Duration::from_millis(5000)).await;
        
        // Human-like delay before starting to type
        let initial_delay = {
            let mut rng = rand::thread_rng();
            rng.gen_range(1000..3000)
        };
        sleep(Duration::from_millis(initial_delay)).await;
        
        // Fill in username
        let username_script = format!(r#"
            (function() {{
                const input = document.querySelector('input[name="username"]');
                if (input) {{
                    input.focus();
                    input.value = '{}';
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                }}
            }})()
        "#, session_info.account.username);
        window.eval(&username_script)?;
        
        // Delay before password
        sleep(Duration::from_millis({
            let mut rng = rand::thread_rng();
            rng.gen_range(500..1500)
        })).await;
        
        // Fill in password
        let password_script = format!(r#"
            (function() {{
                const input = document.querySelector('input[name="password"]');
                if (input) {{
                    input.focus();
                    input.value = '{}';
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                }}
            }})()
        "#, session_info.account.password);
        window.eval(&password_script)?;
        
        // Wait before clicking login
        sleep(Duration::from_millis({
            let mut rng = rand::thread_rng();
            rng.gen_range(500..1500)
        })).await;
        
        // Click login button
        window.eval(r#"
            (function() {
                const button = document.querySelector('button[type="submit"]');
                if (button) {
                    button.click();
                }
            })()
        "#)?;
        
        // Wait for login response
        sleep(Duration::from_millis(5000)).await;
        
        // For now, we'll assume login was successful
        // In a real implementation, you'd use events or other mechanisms to check
        Ok(true)
    }

    pub async fn submit_2fa_code(&self, _session_id: &str, code: &str, window: &WebviewWindow) -> Result<bool> {
        // Fill in 2FA code
        let code_script = format!(r#"
            (function() {{
                const inputs = document.querySelectorAll('input[name="verificationCode"], input[placeholder*="verification"], input[placeholder*="code"], input[type="text"][maxlength="6"]');
                if (inputs.length > 0) {{
                    const input = inputs[0];
                    input.focus();
                    input.value = '{}';
                    input.dispatchEvent(new Event('input', {{ bubbles: true }}));
                }}
            }})()
        "#, code);
        window.eval(&code_script)?;
        
        // Wait a bit
        sleep(Duration::from_millis({
            let mut rng = rand::thread_rng();
            rng.gen_range(500..1000)
        })).await;
        
        // Submit the form
        window.eval(r#"
            (function() {
                const buttons = document.querySelectorAll('button[type="submit"], button:contains("Confirm"), button:contains("Verify")');
                if (buttons.length > 0) {
                    buttons[0].click();
                }
            })()
        "#)?;
        
        // Wait for verification
        sleep(Duration::from_millis(3000)).await;
        
        Ok(true)
    }
} 