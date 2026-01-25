<template>
  <div>
    <a
      v-if="!isAuthenticated"
      href="/login"
      class="text-gray-700 dark:text-gray-300 hover:text-blue-600 dark:hover:text-blue-400 px-3 py-2 rounded-md text-sm font-medium transition-colors"
    >
      Login
    </a>

    <button
      v-else
      @click="handleLogout"
      :disabled="isLoggingOut"
      class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors disabled:opacity-60"
    >
      {{ isLoggingOut ? 'Logging out...' : 'Logout' }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watchEffect } from 'vue';

const isAuthenticated = ref(false);
const isLoggingOut = ref(false);
let handleLogout = async () => {
  if (typeof window !== 'undefined') window.location.href = '/';
};

onMounted(async () => {
  // Use the auth composable for logout (calls API + clears store)
  const { useAuth } = await import('../composables/useAuth');
  const auth = useAuth();

  // Keep logout state in sync
  watchEffect(() => {
    isLoggingOut.value = auth.isLoggingOut;
  });

  handleLogout = async () => {
    try {
      await auth.logout();
      if (typeof window !== 'undefined') window.location.href = '/login';
    } catch (e) {
      // Even if logout fails, clear local state and navigate
      const { useAuthStore } = await import('../stores/auth');
      useAuthStore().clearAuth();
      if (typeof window !== 'undefined') window.location.href = '/login';
    }
  };

  // Watch the Pinia store for authentication state
  const { useAuthStore } = await import('../stores/auth');
  const authStore = useAuthStore();
  watchEffect(() => {
    isAuthenticated.value = !!authStore.isAuthenticated;
  });
});
</script>

<style scoped>
.disabled { opacity: 0.6; }
</style>
