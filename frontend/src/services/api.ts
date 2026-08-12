import axios from 'axios';
import type { LoginRequest, RegisterRequest, AuthTokens, User, Reputation, Report, Evidence, Suggestion, RiskAssessment } from '@/types';

const API_BASE_URL = '/api/v1';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Interceptor pour ajouter le token JWT
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('access_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Interceptor pour gérer les erreurs d'authentification
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('access_token');
      localStorage.removeItem('user');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export const authService = {
  login: async (data: LoginRequest): Promise<AuthTokens> => {
    const response = await api.post<AuthTokens>('/auth/login', data);
    return response.data;
  },

  register: async (data: RegisterRequest): Promise<AuthTokens> => {
    const response = await api.post<AuthTokens>('/auth/register', data);
    return response.data;
  },

  me: async (): Promise<User> => {
    const response = await api.get<User>('/auth/me');
    return response.data;
  },

  logout: () => {
    localStorage.removeItem('access_token');
    localStorage.removeItem('user');
  },
};

export const reputationService = {
  getByEmail: async (email: string): Promise<Reputation> => {
    const response = await api.get<Reputation>(`/reputation/email/${encodeURIComponent(email)}`);
    return response.data;
  },

  getByDomain: async (domain: string): Promise<Reputation> => {
    const response = await api.get<Reputation>(`/reputation/domain/${encodeURIComponent(domain)}`);
    return response.data;
  },

  getByIp: async (ip: string): Promise<Reputation> => {
    const response = await api.get<Reputation>(`/reputation/ip/${encodeURIComponent(ip)}`);
    return response.data;
  },

  search: async (query: string): Promise<Reputation[]> => {
    const response = await api.get<Reputation[]>(`/reputation/search?q=${encodeURIComponent(query)}`);
    return response.data;
  },
};

export const reportService = {
  create: async (data: { target: string; category: string; description: string; evidence?: any[] }): Promise<Report> => {
    const response = await api.post<Report>('/reports', data);
    return response.data;
  },

  getAll: async (): Promise<Report[]> => {
    const response = await api.get<Report[]>('/reports');
    return response.data;
  },

  getById: async (id: string): Promise<Report> => {
    const response = await api.get<Report>(`/reports/${id}`);
    return response.data;
  },
};

export const evidenceService = {
  submit: async (data: { type: string; value: string; source: string; confidence: number }): Promise<Evidence> => {
    const response = await api.post<Evidence>('/evidence', data);
    return response.data;
  },

  getAll: async (): Promise<Evidence[]> => {
    const response = await api.get<Evidence[]>('/evidence');
    return response.data;
  },
};

export const suggestionService = {
  create: async (data: { target: string; suggested_status: string; reason: string; description: string; evidence?: any[] }): Promise<Suggestion> => {
    const response = await api.post<Suggestion>('/suggestions', data);
    return response.data;
  },

  getAll: async (): Promise<Suggestion[]> => {
    const response = await api.get<Suggestion[]>('/suggestions');
    return response.data;
  },
};

export const riskService = {
  evaluate: async (emailRaw: string): Promise<RiskAssessment> => {
    const response = await api.post<RiskAssessment>('/risk/evaluate', { email_raw: emailRaw });
    return response.data;
  },
};

export default api;
