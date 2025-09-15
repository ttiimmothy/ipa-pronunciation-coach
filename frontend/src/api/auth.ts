import apiClient, { type ApiError } from './client';
import type { LoginFormData, RegisterFormData, ChangePasswordFormData } from '../schemas/auth';

export interface User {
  id: string;
  name: string;
  email: string;
  native_language?: string;
  created_at: string;
  updated_at: string;
}

export interface AuthResponse {
  user: User;
  token: string;
}

// Auth API functions with better error handling
export const authApi = {
  async login(data: LoginFormData): Promise<AuthResponse> {
    try {
      const response = await apiClient.post<AuthResponse>('/auth/login', data);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async register(data: RegisterFormData): Promise<AuthResponse> {
    try {
      const response = await apiClient.post<AuthResponse>('/auth/register', data);
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async getMe(): Promise<User> {
    try {
      const response = await apiClient.get<User>('/auth/me');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },

  async changePassword(data: ChangePasswordFormData): Promise<void> {
    try {
      await apiClient.post('/auth/change-password', data);
    } catch (error) {
      throw error as ApiError;
    }
  },

  async logout(): Promise<void> {
    try {
      await apiClient.post('/auth/logout');
    } catch (error) {
      // Don't throw error on logout, just log it
      console.warn('Logout error:', error);
    }
  },

  async refreshToken(): Promise<AuthResponse> {
    try {
      const response = await apiClient.post<AuthResponse>('/auth/refresh');
      return response.data;
    } catch (error) {
      throw error as ApiError;
    }
  },
};
