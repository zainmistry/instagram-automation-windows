import { useState, useEffect } from 'react'
import './App.css'

interface Session {
  id: string
  username: string
  status: string
  created_at: string
  last_activity: string
}

interface Campaign {
  id: string
  name: string
  status: string
  target_count: number
  sent_count: number
}

function App() {
  const [sessions, setSessions] = useState<Session[]>([])
  const [campaigns, setCampaigns] = useState<Campaign[]>([])
  const [activeTab, setActiveTab] = useState<'accounts' | 'campaigns' | 'analytics'>('accounts')

  const [newAccount, setNewAccount] = useState({
    username: '',
    password: '',
    email: '',
    proxy_index: 0
  })

  const [newCampaign, setNewCampaign] = useState({
    name: '',
    message: '',
    targets: '',
    accounts: [] as string[]
  })

  useEffect(() => {
    // Load initial data
    loadSessions()
    loadCampaigns()
  }, [])

  const loadSessions = async () => {
    try {
      // This would connect to the automation engine API
      console.log('Loading sessions...')
      // For now, set empty array to avoid unused setter warning
      setSessions([])
    } catch (error) {
      console.error('Failed to load sessions:', error)
    }
  }

  const loadCampaigns = async () => {
    try {
      // This would connect to the automation engine API
      console.log('Loading campaigns...')
      // For now, set empty array to avoid unused setter warning
      setCampaigns([])
    } catch (error) {
      console.error('Failed to load campaigns:', error)
    }
  }

  const createSession = async () => {
    try {
      console.log('Creating session for:', newAccount.username)
      // Reset form
      setNewAccount({ username: '', password: '', email: '', proxy_index: 0 })
    } catch (error) {
      console.error('Failed to create session:', error)
    }
  }

  const createCampaign = async () => {
    try {
      console.log('Creating campaign:', newCampaign.name)
      // Reset form
      setNewCampaign({ name: '', message: '', targets: '', accounts: [] })
    } catch (error) {
      console.error('Failed to create campaign:', error)
    }
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>Instagram Automation Dashboard</h1>
        <nav className="nav-tabs">
          <button 
            className={activeTab === 'accounts' ? 'active' : ''}
            onClick={() => setActiveTab('accounts')}
          >
            Accounts
          </button>
          <button 
            className={activeTab === 'campaigns' ? 'active' : ''}
            onClick={() => setActiveTab('campaigns')}
          >
            Campaigns
          </button>
          <button 
            className={activeTab === 'analytics' ? 'active' : ''}
            onClick={() => setActiveTab('analytics')}
          >
            Analytics
          </button>
        </nav>
      </header>

      <main className="app-main">
        {activeTab === 'accounts' && (
          <div className="accounts-section">
            <div className="section-header">
              <h2>Account Management</h2>
            </div>
            
            <div className="add-account-form">
              <h3>Add New Account</h3>
              <div className="form-row">
                <input
                  type="text"
                  placeholder="Instagram Username"
                  value={newAccount.username}
                  onChange={(e) => setNewAccount({...newAccount, username: e.target.value})}
                />
                <input
                  type="password"
                  placeholder="Password"
                  value={newAccount.password}
                  onChange={(e) => setNewAccount({...newAccount, password: e.target.value})}
                />
                <input
                  type="email"
                  placeholder="Email"
                  value={newAccount.email}
                  onChange={(e) => setNewAccount({...newAccount, email: e.target.value})}
                />
                <input
                  type="number"
                  placeholder="Proxy Index"
                  value={newAccount.proxy_index}
                  onChange={(e) => setNewAccount({...newAccount, proxy_index: parseInt(e.target.value)})}
                />
                <button onClick={createSession}>Add Account</button>
              </div>
            </div>

            <div className="sessions-list">
              <h3>Active Sessions</h3>
              {sessions.length === 0 ? (
                <p>No active sessions. Add an account to get started.</p>
              ) : (
                <div className="sessions-grid">
                  {sessions.map(session => (
                    <div key={session.id} className="session-card">
                      <h4>{session.username}</h4>
                      <p>Status: <span className={`status ${session.status}`}>{session.status}</span></p>
                      <p>Created: {new Date(session.created_at).toLocaleString()}</p>
                      <div className="session-actions">
                        <button>Login</button>
                        <button>Warmup</button>
                        <button className="danger">Delete</button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}

        {activeTab === 'campaigns' && (
          <div className="campaigns-section">
            <div className="section-header">
              <h2>Campaign Management</h2>
            </div>
            
            <div className="add-campaign-form">
              <h3>Create New Campaign</h3>
              <div className="form-column">
                <input
                  type="text"
                  placeholder="Campaign Name"
                  value={newCampaign.name}
                  onChange={(e) => setNewCampaign({...newCampaign, name: e.target.value})}
                />
                <textarea
                  placeholder="Message Template"
                  value={newCampaign.message}
                  onChange={(e) => setNewCampaign({...newCampaign, message: e.target.value})}
                  rows={4}
                />
                <textarea
                  placeholder="Target Usernames (one per line)"
                  value={newCampaign.targets}
                  onChange={(e) => setNewCampaign({...newCampaign, targets: e.target.value})}
                  rows={6}
                />
                <button onClick={createCampaign}>Create Campaign</button>
              </div>
            </div>

            <div className="campaigns-list">
              <h3>Active Campaigns</h3>
              {campaigns.length === 0 ? (
                <p>No campaigns created yet.</p>
              ) : (
                <div className="campaigns-grid">
                  {campaigns.map(campaign => (
                    <div key={campaign.id} className="campaign-card">
                      <h4>{campaign.name}</h4>
                      <p>Status: <span className={`status ${campaign.status}`}>{campaign.status}</span></p>
                      <p>Progress: {campaign.sent_count}/{campaign.target_count}</p>
                      <div className="campaign-actions">
                        <button>Start</button>
                        <button>Pause</button>
                        <button className="danger">Stop</button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        )}

        {activeTab === 'analytics' && (
          <div className="analytics-section">
            <div className="section-header">
              <h2>Analytics & Monitoring</h2>
            </div>
            
            <div className="analytics-grid">
              <div className="analytics-card">
                <h3>Account Health</h3>
                <div className="metric">
                  <span className="metric-value">0</span>
                  <span className="metric-label">Active Accounts</span>
                </div>
              </div>
              
              <div className="analytics-card">
                <h3>Campaign Performance</h3>
                <div className="metric">
                  <span className="metric-value">0</span>
                  <span className="metric-label">Messages Sent Today</span>
                </div>
              </div>
              
              <div className="analytics-card">
                <h3>System Status</h3>
                <div className="metric">
                  <span className="metric-value status-healthy">Healthy</span>
                  <span className="metric-label">Overall Status</span>
                </div>
              </div>
            </div>
          </div>
        )}
      </main>
    </div>
  )
}

export default App 