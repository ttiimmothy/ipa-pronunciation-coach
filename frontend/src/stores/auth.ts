import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { User } from '../api/auth';

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null);
  const loading = ref(false);

  const isAuthenticated = computed(() => !!user.value);

  const setAuth = (userData: User) => {
    user.value = userData;
  };

  const clearAuth = () => {
    user.value = null;
  };

  const updateUser = (userData: Partial<User>) => {
    if (user.value) {
      user.value = { ...user.value, ...userData };
    }
  };

  return {
    user,
    loading,
    isAuthenticated,
    setAuth,
    clearAuth,
    updateUser,
  };
});
