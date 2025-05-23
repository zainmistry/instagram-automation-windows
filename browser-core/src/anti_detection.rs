use serde::{Deserialize, Serialize};
use tauri::WebviewWindow;
use rand::Rng;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserFingerprint {
    pub user_agent: String,
    pub screen_resolution: (u32, u32),
    pub timezone: String,
    pub language: String,
    pub platform: String,
    pub canvas_fingerprint: String,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub audio_context: String,
    pub fonts: Vec<String>,
    pub plugins: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MouseMovement {
    pub x: f64,
    pub y: f64,
    pub timestamp: u64,
}

pub struct AntiDetectionEngine {
    user_agents: Vec<String>,
    screen_resolutions: Vec<(u32, u32)>,
    timezones: Vec<String>,
    languages: Vec<String>,
}

impl AntiDetectionEngine {
    pub fn new() -> Self {
        Self {
            user_agents: vec![
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15".to_string(),
            ],
            screen_resolutions: vec![
                (1920, 1080),
                (1366, 768),
                (1440, 900),
                (1536, 864),
                (1280, 720),
            ],
            timezones: vec![
                "America/New_York".to_string(),
                "America/Los_Angeles".to_string(),
                "Europe/London".to_string(),
                "Europe/Berlin".to_string(),
                "Asia/Tokyo".to_string(),
            ],
            languages: vec![
                "en-US".to_string(),
                "en-GB".to_string(),
                "de-DE".to_string(),
                "fr-FR".to_string(),
                "es-ES".to_string(),
            ],
        }
    }

    pub async fn setup_browser_fingerprint(&self, window: &WebviewWindow) -> Result<()> {
        let fingerprint = self.generate_fingerprint();
        
        // Inject fingerprint spoofing scripts
        let script = format!(r#"
            (function() {{
                // Override navigator properties
                Object.defineProperty(navigator, 'userAgent', {{
                    get: function() {{ return '{}'; }}
                }});
                
                Object.defineProperty(navigator, 'platform', {{
                    get: function() {{ return '{}'; }}
                }});
                
                Object.defineProperty(navigator, 'language', {{
                    get: function() {{ return '{}'; }}
                }});
                
                Object.defineProperty(navigator, 'languages', {{
                    get: function() {{ return ['{}']; }}
                }});
                
                // Override screen properties
                Object.defineProperty(screen, 'width', {{
                    get: function() {{ return {}; }}
                }});
                
                Object.defineProperty(screen, 'height', {{
                    get: function() {{ return {}; }}
                }});
                
                // Override timezone
                Intl.DateTimeFormat.prototype.resolvedOptions = function() {{
                    return {{
                        timeZone: '{}',
                        locale: '{}'
                    }};
                }};
                
                // Canvas fingerprint spoofing
                HTMLCanvasElement.prototype.getContext = function(contextType, contextAttributes) {{
                    if (contextType === '2d') {{
                        const context = CanvasRenderingContext2D.prototype.getContext.call(this, contextType, contextAttributes);
                        const originalGetImageData = context.getImageData;
                        context.getImageData = function(...args) {{
                            const imageData = originalGetImageData.apply(this, args);
                            // Add slight noise to canvas data
                            for (let i = 0; i < imageData.data.length; i += 4) {{
                                imageData.data[i] += Math.floor(Math.random() * 3) - 1;
                                imageData.data[i + 1] += Math.floor(Math.random() * 3) - 1;
                                imageData.data[i + 2] += Math.floor(Math.random() * 3) - 1;
                            }}
                            return imageData;
                        }};
                        return context;
                    }}
                    return CanvasRenderingContext2D.prototype.getContext.call(this, contextType, contextAttributes);
                }};
                
                // WebGL fingerprint spoofing
                WebGLRenderingContext.prototype.getParameter = function(parameter) {{
                    if (parameter === this.VENDOR) {{
                        return '{}';
                    }}
                    if (parameter === this.RENDERER) {{
                        return '{}';
                    }}
                    return WebGLRenderingContext.prototype.getParameter.call(this, parameter);
                }};
                
                // Audio context spoofing
                if (window.AudioContext || window.webkitAudioContext) {{
                    const AudioContextClass = window.AudioContext || window.webkitAudioContext;
                    const originalCreateOscillator = AudioContextClass.prototype.createOscillator;
                    AudioContextClass.prototype.createOscillator = function() {{
                        const oscillator = originalCreateOscillator.call(this);
                        const originalFrequency = oscillator.frequency.value;
                        oscillator.frequency.value = originalFrequency + (Math.random() - 0.5) * 0.001;
                        return oscillator;
                    }};
                }}
                
                // Remove automation indicators
                Object.defineProperty(navigator, 'webdriver', {{
                    get: function() {{ return undefined; }}
                }});
                
                Object.defineProperty(window, 'chrome', {{
                    get: function() {{ 
                        return {{
                            runtime: {{}}
                        }};
                    }}
                }});
                
                // Hide selenium/automation properties
                ['__selenium_unwrapped', '__webdriver_evaluate', '__selenium_evaluate', '__fxdriver_evaluate', '__driver_unwrapped', '__webdriver_script_function', '__webdriver_script_func', '__webdriver_script_fn', '__driver_evaluate', '__selenium_unwrapped', '__fxdriver_unwrapped'].forEach(prop => {{
                    Object.defineProperty(window, prop, {{
                        get: function() {{ return undefined; }},
                        set: function() {{}}
                    }});
                }});
                
                console.log('Anti-detection measures applied successfully');
            }})();
        "#,
            fingerprint.user_agent,
            fingerprint.platform,
            fingerprint.language,
            fingerprint.language,
            fingerprint.screen_resolution.0,
            fingerprint.screen_resolution.1,
            fingerprint.timezone,
            fingerprint.language,
            fingerprint.webgl_vendor,
            fingerprint.webgl_renderer,
        );
        
        window.eval(&script).map_err(|e| anyhow::anyhow!("Failed to inject anti-detection script: {}", e))?;
        
        Ok(())
    }

    fn generate_fingerprint(&self) -> BrowserFingerprint {
        let mut rng = rand::thread_rng();
        
        BrowserFingerprint {
            user_agent: self.user_agents[rng.gen_range(0..self.user_agents.len())].clone(),
            screen_resolution: self.screen_resolutions[rng.gen_range(0..self.screen_resolutions.len())],
            timezone: self.timezones[rng.gen_range(0..self.timezones.len())].clone(),
            language: self.languages[rng.gen_range(0..self.languages.len())].clone(),
            platform: if rng.gen_bool(0.7) { "MacIntel".to_string() } else { "Win32".to_string() },
            canvas_fingerprint: format!("canvas_{}", rng.gen::<u32>()),
            webgl_vendor: "Intel Inc.".to_string(),
            webgl_renderer: format!("Intel(R) Iris(TM) Graphics {}", rng.gen_range(5000..6000)),
            audio_context: format!("audio_{}", rng.gen::<u32>()),
            fonts: vec![
                "Arial".to_string(),
                "Helvetica".to_string(),
                "Times New Roman".to_string(),
                "Courier New".to_string(),
            ],
            plugins: vec![
                "Chrome PDF Plugin".to_string(),
                "Chrome PDF Viewer".to_string(),
                "Native Client".to_string(),
            ],
        }
    }

    pub fn generate_human_mouse_movement(&self, start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Vec<MouseMovement> {
        let mut movements = Vec::new();
        let mut rng = rand::thread_rng();
        
        let distance = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
        let steps = (distance / 5.0).max(10.0) as i32;
        
        let mut current_time = 0u64;
        
        for i in 0..=steps {
            let progress = i as f64 / steps as f64;
            
            // Bezier curve for natural movement
            let t = progress;
            let control_x = start_x + (end_x - start_x) * 0.5 + rng.gen_range(-20.0..20.0);
            let control_y = start_y + (end_y - start_y) * 0.5 + rng.gen_range(-20.0..20.0);
            
            let x = (1.0 - t).powi(2) * start_x + 2.0 * (1.0 - t) * t * control_x + t.powi(2) * end_x;
            let y = (1.0 - t).powi(2) * start_y + 2.0 * (1.0 - t) * t * control_y + t.powi(2) * end_y;
            
            // Add small random variations
            let x = x + rng.gen_range(-2.0..2.0);
            let y = y + rng.gen_range(-2.0..2.0);
            
            // Variable timing between movements
            current_time += rng.gen_range(10..30);
            
            movements.push(MouseMovement {
                x,
                y,
                timestamp: current_time,
            });
        }
        
        movements
    }

    pub async fn simulate_reading_pause(&self) -> Result<()> {
        let pause_duration = {
            let mut rng = rand::thread_rng();
            rng.gen_range(2000..5000)
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(pause_duration)).await;
        Ok(())
    }
} 