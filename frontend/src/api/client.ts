import axios from 'axios';
import type { AxiosResponse, AxiosError } from 'axios';

const API_BASE_URL = 'http://localhost:3000';

export interface ApiError {
  message: string;
  status?: number;
  errors?: Record<string, string[]>;
}

export interface ApiResponse<T = any> {
  data: T;
  message?: string;
  success: boolean;
}

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  // timeout: 10000, // 10 seconds timeout
  withCredentials: true
});

// Request interceptor for debugging
apiClient.interceptors.request.use(
  (config) => {
    // Add request timestamp for debugging
    (config as any).metadata = { startTime: new Date() };
    
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Response interceptor to handle responses and errors
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    // Log successful requests in development
    if (import.meta.env.DEV) {
      const duration = new Date().getTime() - (response.config as any).metadata?.startTime?.getTime();
      console.log(`✅ ${response.config.method?.toUpperCase()} ${response.config.url} - ${response.status} (${duration}ms)`);
    }
    return response;
  },
  (error: AxiosError) => {
    // Log errors in development
    if (import.meta.env.DEV) {
      const duration = (error.config as any)?.metadata?.startTime 
        ? new Date().getTime() - (error.config as any).metadata.startTime.getTime()
        : 0;
      console.error(`❌ ${error.config?.method?.toUpperCase()} ${error.config?.url} - ${error.response?.status} (${duration}ms)`, error.response?.data);
    }

    // Handle different error types
    if (error.response?.status === 401) {
      // Token expired or invalid - server will clear cookie
      if (typeof window !== 'undefined' && window.location.pathname !== '/login') {
        window.location.href = '/login';
      }
    } else if (error.response?.status === 403) {
      // Forbidden - user doesn't have permission
      console.error('Access forbidden');
    } else if (error.response?.status === 404) {
      // Not found
      console.error('Resource not found');
    } else if (error.response?.status && error.response.status >= 500) {
      // Server error
      console.error('Server error occurred');
    }

    // Transform error to our ApiError format
    const apiError: ApiError = {
      message: (error.response?.data as any)?.message || error.message || 'An error occurred',
      status: error.response?.status,
      errors: (error.response?.data as any)?.errors,
    };

    return Promise.reject(apiError);
  }
);

export default apiClient;
