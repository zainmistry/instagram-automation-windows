# Instagram Automation Tool

A sophisticated Instagram automation tool with advanced anti-detection capabilities, designed to handle multiple accounts with human-like behavior simulation.

## 🚀 Features

- **Anti-Detection Browser**: Custom Tauri-based browser with advanced fingerprint spoofing
- **Multi-Account Management**: Handle up to 20 Instagram accounts simultaneously
- **Proxy Integration**: Built-in proxy rotation and health monitoring
- **Human Behavior Simulation**: ML-driven behavior patterns to mimic real users
- **Account Warmup**: Gradual activity increase to build account trust
- **DM Marketing**: Automated but natural-looking direct message campaigns
- **2FA Support**: Manual intervention system for verification codes
- **Safety Monitoring**: Real-time detection and prevention of account flags

## 🏗️ Architecture

```
├── browser-core/          # Custom Tauri browser application
├── automation-engine/     # Python backend for automation logic
├── dashboard/            # React-based management interface
├── database/             # SQLite database files
├── config/               # Configuration files
└── docs/                 # Documentation
```

## 🛠️ Technology Stack

- **Browser**: Tauri (Rust + WebView2)
- **Backend**: Python 3.11+
- **Frontend**: React + TypeScript + Vite
- **Database**: SQLite
- **Proxy Management**: Custom rotation system
- **Behavior Engine**: scikit-learn + custom algorithms

## 📋 Prerequisites

- Node.js 18+
- Python 3.11+
- Rust 1.70+
- Git

## 🚀 Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd instagram-automation-tool
   ```

2. **Install dependencies**
   ```bash
   # Install Tauri CLI
   cargo install tauri-cli
   
   # Install Python dependencies
   pip install -r requirements.txt
   
   # Install Node.js dependencies
   npm install
   ```

3. **Configure proxies**
   ```bash
   cp config/proxies.example.json config/proxies.json
   # Edit config/proxies.json with your proxy list
   ```

4. **Start the application**
   ```bash
   # Start the automation engine
   python automation-engine/main.py
   
   # Start the browser (in another terminal)
   cargo tauri dev
   
   # Start the dashboard (in another terminal)
   cd dashboard && npm run dev
   ```

## ⚙️ Configuration

### Proxy Setup
Edit `config/proxies.json`:
```json
{
  "proxies": [
    {
      "host": "proxy1.example.com",
      "port": 8080,
      "username": "user1",
      "password": "pass1",
      "protocol": "http"
    }
  ]
}
```

### Account Configuration
Add accounts in `config/accounts.json`:
```json
{
  "accounts": [
    {
      "username": "account1",
      "password": "password1",
      "email": "email1@example.com",
      "proxy_index": 0
    }
  ]
}
```

## 🔒 Security & Anti-Detection

- **Fingerprint Spoofing**: Canvas, WebGL, audio context randomization
- **Behavioral Patterns**: Natural mouse movements, typing patterns
- **Timing Variation**: Human-like delays between actions
- **Session Management**: Persistent login states with secure storage
- **Rate Limiting**: Conservative activity patterns to avoid detection

## 📊 Monitoring

The tool includes comprehensive monitoring:
- Account health status
- Proxy performance metrics
- Activity logs and analytics
- Real-time alerts for issues

## ⚠️ Important Notes

- Use this tool responsibly and in accordance with Instagram's Terms of Service
- Always use high-quality proxies to protect account safety
- Start with conservative settings and gradually increase activity
- Monitor accounts regularly for any flags or restrictions

## 🤝 Contributing

Please read our contributing guidelines before submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## ⚠️ Disclaimer

This tool is for educational purposes only. Users are responsible for ensuring compliance with Instagram's Terms of Service and applicable laws. 