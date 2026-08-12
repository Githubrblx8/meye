import React from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Shield, Mail, AlertTriangle, Eye, FileText, XCircle } from 'lucide-react';

interface StatCardProps {
  title: string;
  value: string | number;
  icon: React.ElementType;
  description?: string;
}

function StatCard({ title, value, icon: Icon, description }: StatCardProps) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">{title}</CardTitle>
        <Icon className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        {description && <p className="text-xs text-muted-foreground">{description}</p>}
      </CardContent>
    </Card>
  );
}

export function DashboardOverview() {
  // Mock data - will be replaced with real API calls
  const stats = {
    emailsAnalyzed: 12482,
    blocked: 231,
    allowed: 11890,
    underWatch: 361,
    reports: 89,
    threatsDetected: 42,
  };

  const recentThreats = [
    { identity: 'phishing@example.com', status: 'BLOCKED', riskScore: 92 },
    { identity: 'suspicious.net', status: 'WATCH', riskScore: 71 },
    { identity: 'trusted-domain.com', status: 'SAFE', riskScore: 4 },
    { identity: 'malware-host.org', status: 'COMPROMISED', riskScore: 95 },
    { identity: 'newsletter@legit.com', status: 'SAFE', riskScore: 8 },
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'SAFE': return 'bg-green-500';
      case 'UNKNOWN': return 'bg-gray-400';
      case 'WATCH': return 'bg-orange-500';
      case 'BLOCKED': return 'bg-red-500';
      case 'COMPROMISED': return 'bg-purple-500';
      default: return 'bg-gray-400';
    }
  };

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">Security Overview</h2>
        <p className="text-muted-foreground">Real-time email security metrics and threat analysis</p>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <StatCard
          title="Emails Analyzed"
          value={stats.emailsAnalyzed.toLocaleString()}
          icon={Mail}
          description="+12.5% from last week"
        />
        <StatCard
          title="Blocked"
          value={stats.blocked.toLocaleString()}
          icon={XCircle}
          description="High risk detected"
        />
        <StatCard
          title="Under Watch"
          value={stats.underWatch.toLocaleString()}
          icon={Eye}
          description="Monitoring active"
        />
        <StatCard
          title="Threats Detected"
          value={stats.threatsDetected.toLocaleString()}
          icon={AlertTriangle}
          description="This week"
        />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-7">
        <Card className="col-span-4">
          <CardHeader>
            <CardTitle>Recent Threat Activity</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {recentThreats.map((threat, index) => (
                <div key={index} className="flex items-center justify-between border-b pb-4 last:border-0">
                  <div className="flex items-center gap-3">
                    <div className={`h-2 w-2 rounded-full ${getStatusColor(threat.status)}`} />
                    <div>
                      <p className="font-medium">{threat.identity}</p>
                      <p className="text-xs text-muted-foreground">Risk Score: {threat.riskScore}</p>
                    </div>
                  </div>
                  <span className={`rounded-md px-2 py-1 text-xs font-semibold ${
                    threat.status === 'SAFE' ? 'bg-green-100 text-green-800' :
                    threat.status === 'WATCH' ? 'bg-orange-100 text-orange-800' :
                    threat.status === 'BLOCKED' ? 'bg-red-100 text-red-800' :
                    'bg-purple-100 text-purple-800'
                  }`}>
                    {threat.status}
                  </span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        <Card className="col-span-3">
          <CardHeader>
            <CardTitle>Risk Distribution</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm">Low Risk (0-30)</span>
                <div className="flex items-center gap-2">
                  <div className="h-2 w-32 rounded-full bg-green-500" style={{ width: `${(stats.allowed / stats.emailsAnalyzed) * 100}%` }} />
                  <span className="text-sm font-medium">{Math.round((stats.allowed / stats.emailsAnalyzed) * 100)}%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Medium Risk (31-60)</span>
                <div className="flex items-center gap-2">
                  <div className="h-2 w-32 rounded-full bg-orange-500" style={{ width: `${(stats.underWatch / stats.emailsAnalyzed) * 100}%` }} />
                  <span className="text-sm font-medium">{Math.round((stats.underWatch / stats.emailsAnalyzed) * 100)}%</span>
                </div>
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">High Risk (61-100)</span>
                <div className="flex items-center gap-2">
                  <div className="h-2 w-32 rounded-full bg-red-500" style={{ width: `${(stats.blocked / stats.emailsAnalyzed) * 100}%` }} />
                  <span className="text-sm font-medium">{Math.round((stats.blocked / stats.emailsAnalyzed) * 100)}%</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
