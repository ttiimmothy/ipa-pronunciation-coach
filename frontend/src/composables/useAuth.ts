import { useMutation, useQuery, useQueryClient } from '@tanstack/vue-query';
import { authApi } from '../api/auth';
import { useAuthStore } from '../stores/auth';
import type { ApiError } from '../api/client';
import {ChangePasswordFormData, LoginFormData, RegisterFormData} from "../schemas/auth";

export const useAuth = () => {
  const authStore = useAuthStore();
  const queryClient = useQueryClient();

  // Get current user
  const { 
    data: user, 
    isLoading: isLoadingUser, 
    error: userError,
    refetch: refetchUser 
  } = useQuery({
    queryKey: ['auth', 'user'],
    queryFn: authApi.getMe,
    enabled: authStore.isAuthenticated,
    retry: (failureCount, error: ApiError) => {
      // Don't retry on auth errors
      if (error?.status === 401 || error?.status === 403) {
        return false;
      }
      return failureCount < 2;
    },
    staleTime: 5 * 60 * 1000, // 5 minutes
  });

  // Login mutation
  const loginMutation = useMutation({
    mutationFn: authApi.login,
    onSuccess: (data) => {
      authStore.setAuth(data.user);
      queryClient.setQueryData(['auth', 'user'], data.user);
      // Invalidate related queries
      queryClient.invalidateQueries({ queryKey: ['practice'] });
      queryClient.invalidateQueries({ queryKey: ['vocabulary'] });
    },
    onError: (error: ApiError) => {
      console.error('Login failed:', error);
    },
  });

  // Register mutation
  const registerMutation = useMutation({
    mutationFn: authApi.register,
    onSuccess: (data) => {
      authStore.setAuth(data.user);
      queryClient.setQueryData(['auth', 'user'], data.user);
      // Invalidate related queries
      queryClient.invalidateQueries({ queryKey: ['practice'] });
      queryClient.invalidateQueries({ queryKey: ['vocabulary'] });
    },
    onError: (error: ApiError) => {
      console.error('Registration failed:', error);
    },
  });

  // Change password mutation
  const changePasswordMutation = useMutation({
    mutationFn: authApi.changePassword,
    onSuccess: () => {
      // Optionally show success message
      console.log('Password changed successfully');
    },
    onError: (error: ApiError) => {
      console.error('Password change failed:', error);
    },
  });

  // Logout mutation
  const logoutMutation = useMutation({
    mutationFn: authApi.logout,
    onSuccess: () => {
      authStore.clearAuth();
      queryClient.clear();
    },
    onError: (error: ApiError) => {
      // Even if logout fails on server, clear local state
      console.warn('Logout error:', error);
      authStore.clearAuth();
      queryClient.clear();
    },
  });

  const login = (data: LoginFormData) => loginMutation.mutate(data);
  const register = (data: RegisterFormData) => registerMutation.mutate(data);
  const changePassword = (data: ChangePasswordFormData) => changePasswordMutation.mutate(data);
  const logout = () => logoutMutation.mutate();

  return {
    // Data
    user: user.value,
    isLoadingUser,
    userError,
    refetchUser,
    
    // Login
    login,
    isLoggingIn: loginMutation.isPending.value,
    loginError: loginMutation.error.value,
    loginSuccess: loginMutation.isSuccess.value,
    
    // Register
    register,
    isRegistering: registerMutation.isPending.value,
    registerError: registerMutation.error.value,
    registerSuccess: registerMutation.isSuccess.value,
    
    // Change Password
    changePassword,
    isChangingPassword: changePasswordMutation.isPending.value,
    changePasswordError: changePasswordMutation.error.value,
    changePasswordSuccess: changePasswordMutation.isSuccess.value,
    
    // Logout
    logout,
    isLoggingOut: logoutMutation.isPending.value,
  };
};
