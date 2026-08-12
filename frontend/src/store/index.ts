import { create } from 'zustand';
import type { User, Reputation, Report, Evidence, Suggestion } from '@/types';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  setUser: (user: User | null) => void;
  logout: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  isAuthenticated: false,
  setUser: (user) => set({ user, isAuthenticated: !!user }),
  logout: () => set({ user: null, isAuthenticated: false }),
}));

interface DashboardState {
  stats: {
    emailsAnalyzed: number;
    blocked: number;
    allowed: number;
    underWatch: number;
    reports: number;
    threatsDetected: number;
  };
  recentThreats: Array<{ identity: string; status: string; riskScore: number }>;
  setStats: (stats: DashboardState['stats']) => void;
  setRecentThreats: (threats: DashboardState['recentThreats']) => void;
}

export const useDashboardStore = create<DashboardState>((set) => ({
  stats: {
    emailsAnalyzed: 0,
    blocked: 0,
    allowed: 0,
    underWatch: 0,
    reports: 0,
    threatsDetected: 0,
  },
  recentThreats: [],
  setStats: (stats) => set({ stats }),
  setRecentThreats: (threats) => set({ recentThreats: threats }),
}));

interface ReputationState {
  currentReputation: Reputation | null;
  searchHistory: string[];
  setCurrentReputation: (reputation: Reputation | null) => void;
  addToSearchHistory: (query: string) => void;
}

export const useReputationStore = create<ReputationState>((set) => ({
  currentReputation: null,
  searchHistory: [],
  setCurrentReputation: (reputation) => set({ currentReputation: reputation }),
  addToSearchHistory: (query) =>
    set((state) => ({
      searchHistory: [query, ...state.searchHistory.filter((q) => q !== query)].slice(0, 10),
    })),
}));

interface ReportState {
  reports: Report[];
  currentReport: Report | null;
  setReports: (reports: Report[]) => void;
  setCurrentReport: (report: Report | null) => void;
  addReport: (report: Report) => void;
}

export const useReportStore = create<ReportState>((set) => ({
  reports: [],
  currentReport: null,
  setReports: (reports) => set({ reports }),
  setCurrentReport: (report) => set({ currentReport: report }),
  addReport: (report) => set((state) => ({ reports: [report, ...state.reports] })),
}));

interface ModerationState {
  pendingReports: Report[];
  pendingSuggestions: Suggestion[];
  setPendingReports: (reports: Report[]) => void;
  setPendingSuggestions: (suggestions: Suggestion[]) => void;
  approveReport: (id: string) => void;
  rejectReport: (id: string) => void;
}

export const useModerationStore = create<ModerationState>((set) => ({
  pendingReports: [],
  pendingSuggestions: [],
  setPendingReports: (reports) => set({ pendingReports: reports }),
  setPendingSuggestions: (suggestions) => set({ pendingSuggestions: suggestions }),
  approveReport: (id) =>
    set((state) => ({
      pendingReports: state.pendingReports.filter((r) => r.id !== id),
    })),
  rejectReport: (id) =>
    set((state) => ({
      pendingReports: state.pendingReports.filter((r) => r.id !== id),
    })),
}));
