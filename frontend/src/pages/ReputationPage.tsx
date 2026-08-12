import React, { useState } from 'react';
import { Search, Shield, AlertTriangle, CheckCircle, XCircle, Clock, TrendingUp, Activity, Database, FileText, ExternalLink, ArrowRight, Filter, Download } from 'lucide-react';
import api from '../lib/api';
import { useAuthStore } from '../stores/authStore';

interface ReputationData {
  identity: string;
  type: 'email' | 'domain' | 'ip' | 'url';
  status: 'SAFE' | 'UNKNOWN' | 'WATCH' | 'BLOCKED' | 'COMPROMISED';
  risk_score: number;
  confidence: number;
  first_seen: string;
  last_seen: string;
  reports_count: number;
  confirmed_incidents: number;
  authentication_results?: {
    spf?: string;
    dkim?: string;
    dmarc?: string;
  };
  related_infrastructure?: string[];
  history?: Array<{
    timestamp: string;
    event: string;
    previous_status: string;
    new_status: string;
  }>;
}

const statusConfig = {
  SAFE: { color: 'bg-green-500', textColor: 'text-green-400', icon: CheckCircle, label: 'Safe' },
  UNKNOWN: { color: 'bg-gray-500', textColor: 'text-gray-400', icon: Clock, label: 'Unknown' },
  WATCH: { color: 'bg-orange-500', textColor: 'text-orange-400', icon: AlertTriangle, label: 'Watch' },
  BLOCKED: { color: 'bg-red-500', textColor: 'text-red-400', icon: XCircle, label: 'Blocked' },
  COMPROMISED: { color: 'bg-purple-500', textColor: 'text-purple-400', icon: Shield, label: 'Compromised' }
};

export default function ReputationPage() {
  const [searchQuery, setSearchQuery] = useState('');
  const [searchType, setSearchType] = useState<'email' | 'domain' | 'ip' | 'url'>('email');
  const [reputation, setReputation] = useState<ReputationData | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'overview' | 'evidence' | 'history' | 'related'>('overview');
  const { user } = useAuthStore();

  const handleSearch = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!searchQuery.trim()) return;

    setLoading(true);
    setError(null);
    
    try {
      let endpoint = '';
      switch (searchType) {
        case 'email':
          endpoint = `/api/v1/reputation/email/${encodeURIComponent(searchQuery)}`;
          break;
        case 'domain':
          endpoint = `/api/v1/reputation/domain/${encodeURIComponent(searchQuery)}`;
          break;
        case 'ip':
          endpoint = `/api/v1/reputation/ip/${encodeURIComponent(searchQuery)}`;
          break;
        case 'url':
          endpoint = `/api/v1/reputation/url/${encodeURIComponent(searchQuery)}`;
          break;
      }

      const response = await api.get(endpoint);
      setReputation(response.data);
    } catch (err: any) {
      setError(err.response?.data?.detail || 'Failed to fetch reputation data');
      setReputation(null);
    } finally {
      setLoading(false);
    }
  };

  const getStatusIcon = (status: string) => {
    const config = statusConfig[status as keyof typeof statusConfig];
    const Icon = config?.icon || Clock;
    return <Icon className={`w-6 h-6 ${config?.textColor}`} />;
  };

  const getRiskLevel = (score: number) => {
    if (score <= 30) return { label: 'Low', color: 'text-green-400' };
    if (score <= 60) return { label: 'Medium', color: 'text-orange-400' };
    return { label: 'High', color: 'text-red-400' };
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-white">Reputation Lookup</h1>
          <p className="text-gray-400 mt-1">Check the reputation of emails, domains, IPs, and URLs</p>
        </div>
        {user && ['moderator', 'administrator'].includes(user.role) && (
          <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center gap-2">
            <FileText className="w-4 h-4" />
            Bulk Lookup
          </button>
        )}
      </div>

      {/* Search Box */}
      <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
        <form onSubmit={handleSearch} className="space-y-4">
          <div className="flex gap-4">
            <div className="flex-1">
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Identity to check
              </label>
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-5 h-5 text-gray-400" />
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder={
                    searchType === 'email' ? 'user@example.com' :
                    searchType === 'domain' ? 'example.com' :
                    searchType === 'ip' ? '192.168.1.1' :
                    'https://example.com/path'
                  }
                  className="w-full pl-10 pr-4 py-3 bg-gray-900 border border-gray-600 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
            </div>
            
            <div className="w-48">
              <label className="block text-sm font-medium text-gray-300 mb-2">
                Type
              </label>
              <select
                value={searchType}
                onChange={(e) => setSearchType(e.target.value as any)}
                className="w-full px-4 py-3 bg-gray-900 border border-gray-600 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              >
                <option value="email">Email</option>
                <option value="domain">Domain</option>
                <option value="ip">IP Address</option>
                <option value="url">URL</option>
              </select>
            </div>
            
            <div className="flex items-end">
              <button
                type="submit"
                disabled={loading}
                className="px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-medium rounded-lg transition-colors flex items-center gap-2"
              >
                {loading ? (
                  <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
                ) : (
                  <>
                    <Search className="w-5 h-5" />
                    Search
                  </>
                )}
              </button>
            </div>
          </div>
        </form>
      </div>

      {/* Error Display */}
      {error && (
        <div className="bg-red-900/20 border border-red-800 rounded-xl p-4 flex items-center gap-3">
          <XCircle className="w-5 h-5 text-red-400" />
          <span className="text-red-400">{error}</span>
        </div>
      )}

      {/* Results */}
      {reputation && (
        <div className="space-y-6">
          {/* Status Cards */}
          <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-2">
                <Shield className="w-5 h-5 text-gray-400" />
                <span className="text-gray-400 text-sm">Status</span>
              </div>
              <div className="flex items-center gap-2">
                {getStatusIcon(reputation.status)}
                <span className={`text-xl font-bold ${statusConfig[reputation.status]?.textColor}`}>
                  {statusConfig[reputation.status]?.label}
                </span>
              </div>
            </div>

            <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-2">
                <TrendingUp className="w-5 h-5 text-gray-400" />
                <span className="text-gray-400 text-sm">Risk Score</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-2xl font-bold text-white">{reputation.risk_score}</span>
                <span className={`text-sm ${getRiskLevel(reputation.risk_score).color}`}>
                  {getRiskLevel(reputation.risk_score).label}
                </span>
              </div>
              <div className="mt-2 w-full bg-gray-700 rounded-full h-2">
                <div 
                  className={`h-2 rounded-full transition-all ${
                    reputation.risk_score <= 30 ? 'bg-green-500' :
                    reputation.risk_score <= 60 ? 'bg-orange-500' : 'bg-red-500'
                  }`}
                  style={{ width: `${reputation.risk_score}%` }}
                />
              </div>
            </div>

            <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-2">
                <Activity className="w-5 h-5 text-gray-400" />
                <span className="text-gray-400 text-sm">Confidence</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-2xl font-bold text-white">
                  {(reputation.confidence * 100).toFixed(0)}%
                </span>
              </div>
              <div className="mt-2 w-full bg-gray-700 rounded-full h-2">
                <div 
                  className="h-2 rounded-full bg-blue-500 transition-all"
                  style={{ width: `${reputation.confidence * 100}%` }}
                />
              </div>
            </div>

            <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
              <div className="flex items-center gap-3 mb-2">
                <Database className="w-5 h-5 text-gray-400" />
                <span className="text-gray-400 text-sm">Reports</span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-2xl font-bold text-white">{reputation.reports_count}</span>
                <span className="text-sm text-gray-400">
                  {reputation.confirmed_incidents} confirmed
                </span>
              </div>
            </div>
          </div>

          {/* Tabs */}
          <div className="border-b border-gray-700">
            <nav className="flex gap-6">
              {[
                { id: 'overview', label: 'Overview' },
                { id: 'evidence', label: 'Evidence' },
                { id: 'history', label: 'History' },
                { id: 'related', label: 'Related' }
              ].map((tab) => (
                <button
                  key={tab.id}
                  onClick={() => setActiveTab(tab.id as any)}
                  className={`py-3 px-1 border-b-2 font-medium text-sm transition-colors ${
                    activeTab === tab.id
                      ? 'border-blue-500 text-blue-400'
                      : 'border-transparent text-gray-400 hover:text-gray-300'
                  }`}
                >
                  {tab.label}
                </button>
              ))}
            </nav>
          </div>

          {/* Tab Content */}
          <div className="bg-gray-800 border border-gray-700 rounded-xl p-6">
            {activeTab === 'overview' && (
              <div className="space-y-6">
                <div>
                  <h3 className="text-lg font-semibold text-white mb-4">Identity Information</h3>
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <span className="text-gray-400 text-sm">Identity</span>
                      <p className="text-white font-mono mt-1">{reputation.identity}</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">Type</span>
                      <p className="text-white capitalize mt-1">{reputation.type}</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">First Seen</span>
                      <p className="text-white mt-1">{new Date(reputation.first_seen).toLocaleDateString()}</p>
                    </div>
                    <div>
                      <span className="text-gray-400 text-sm">Last Seen</span>
                      <p className="text-white mt-1">{new Date(reputation.last_seen).toLocaleDateString()}</p>
                    </div>
                  </div>
                </div>

                {reputation.authentication_results && (
                  <div>
                    <h3 className="text-lg font-semibold text-white mb-4">Authentication Results</h3>
                    <div className="grid grid-cols-3 gap-4">
                      {Object.entries(reputation.authentication_results).map(([key, value]) => (
                        <div key={key} className="bg-gray-900 rounded-lg p-4">
                          <span className="text-gray-400 text-sm uppercase">{key}</span>
                          <div className="flex items-center gap-2 mt-2">
                            {value === 'pass' ? (
                              <CheckCircle className="w-5 h-5 text-green-400" />
                            ) : value === 'fail' ? (
                              <XCircle className="w-5 h-5 text-red-400" />
                            ) : (
                              <Clock className="w-5 h-5 text-gray-400" />
                            )}
                            <span className={`font-medium capitalize ${
                              value === 'pass' ? 'text-green-400' :
                              value === 'fail' ? 'text-red-400' : 'text-gray-400'
                            }`}>
                              {value || 'N/A'}
                            </span>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Actions */}
                <div className="flex gap-3 pt-4 border-t border-gray-700">
                  {['moderator', 'administrator'].includes(user?.role || '') && (
                    <>
                      <button className="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-lg transition-colors flex items-center gap-2">
                        <CheckCircle className="w-4 h-4" />
                        Mark as Safe
                      </button>
                      <button className="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded-lg transition-colors flex items-center gap-2">
                        <XCircle className="w-4 h-4" />
                        Block
                      </button>
                    </>
                  )}
                  <button className="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg transition-colors flex items-center gap-2">
                    <AlertTriangle className="w-4 h-4" />
                    Report
                  </button>
                  <button className="px-4 py-2 bg-gray-600 hover:bg-gray-700 text-white rounded-lg transition-colors flex items-center gap-2">
                    <FileText className="w-4 h-4" />
                    Suggest Classification
                  </button>
                </div>
              </div>
            )}

            {activeTab === 'evidence' && (
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <h3 className="text-lg font-semibold text-white">Evidence & Reports</h3>
                  <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center gap-2 text-sm">
                    <Download className="w-4 h-4" />
                    Export Evidence
                  </button>
                </div>
                <div className="text-center py-12">
                  <FileText className="w-12 h-12 text-gray-600 mx-auto mb-4" />
                  <p className="text-gray-400">No evidence submitted yet</p>
                  <button className="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors text-sm">
                    Submit Evidence
                  </button>
                </div>
              </div>
            )}

            {activeTab === 'history' && (
              <div className="space-y-4">
                <h3 className="text-lg font-semibold text-white">Status History</h3>
                {reputation.history && reputation.history.length > 0 ? (
                  <div className="space-y-3">
                    {reputation.history.map((event, index) => (
                      <div key={index} className="flex items-start gap-4 p-4 bg-gray-900 rounded-lg">
                        <Clock className="w-5 h-5 text-gray-400 mt-0.5" />
                        <div className="flex-1">
                          <p className="text-white">{event.event}</p>
                          <div className="flex items-center gap-2 mt-1 text-sm">
                            <span className="text-gray-400">{event.previous_status}</span>
                            <ArrowRight className="w-4 h-4 text-gray-500" />
                            <span className={statusConfig[event.new_status as keyof typeof statusConfig]?.textColor}>
                              {event.new_status}
                            </span>
                          </div>
                          <p className="text-gray-500 text-xs mt-1">
                            {new Date(event.timestamp).toLocaleString()}
                          </p>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-12">
                    <Clock className="w-12 h-12 text-gray-600 mx-auto mb-4" />
                    <p className="text-gray-400">No history available</p>
                  </div>
                )}
              </div>
            )}

            {activeTab === 'related' && (
              <div className="space-y-4">
                <h3 className="text-lg font-semibold text-white">Related Infrastructure</h3>
                {reputation.related_infrastructure && reputation.related_infrastructure.length > 0 ? (
                  <div className="grid grid-cols-2 gap-3">
                    {reputation.related_infrastructure.map((item, index) => (
                      <div key={index} className="flex items-center gap-3 p-3 bg-gray-900 rounded-lg">
                        <ExternalLink className="w-4 h-4 text-gray-400" />
                        <span className="text-white font-mono text-sm">{item}</span>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="text-center py-12">
                    <Database className="w-12 h-12 text-gray-600 mx-auto mb-4" />
                    <p className="text-gray-400">No related infrastructure found</p>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      {!reputation && !loading && !error && (
        <div className="text-center py-16">
          <Search className="w-16 h-16 text-gray-600 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-white mb-2">Search for an Identity</h3>
          <p className="text-gray-400 max-w-md mx-auto">
            Enter an email address, domain, IP address, or URL to check its reputation score and history.
          </p>
        </div>
      )}
    </div>
  );
}
