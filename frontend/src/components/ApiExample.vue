<template>
  <div class="p-6 space-y-6">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
      API Integration Example
    </h2>

    <!-- Auth Section -->
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 shadow">
      <h3 class="text-lg font-semibold mb-4">Authentication</h3>
      <div class="space-y-2">
        <div v-if="isLoadingUser" class="text-blue-600">Loading user...</div>
        <div v-else-if="user" class="text-green-600">
          Logged in as: {{ user.name }} ({{ user.email }})
        </div>
        <div v-else class="text-gray-600">Not logged in</div>
        
        <div v-if="loginError" class="text-red-600">
          Login error: {{ loginError.message }}
        </div>
        
        <button 
          @click="handleLogin" 
          :disabled="isLoggingIn"
          class="bg-blue-600 text-white px-4 py-2 rounded disabled:opacity-50"
        >
          {{ isLoggingIn ? 'Logging in...' : 'Test Login' }}
        </button>
      </div>
    </div>

    <!-- Vocabulary Section -->
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 shadow">
      <h3 class="text-lg font-semibold mb-4">Vocabulary</h3>
      <div class="space-y-2">
        <div v-if="isLoadingVocabulary" class="text-blue-600">Loading vocabulary...</div>
        <div v-else class="text-gray-600">
          Found {{ vocabularyWords.length }} words
        </div>
        
        <div v-if="vocabularyError" class="text-red-600">
          Vocabulary error: {{ vocabularyError.message }}
        </div>
        
        <button 
          @click="loadVocabulary" 
          :disabled="isLoadingVocabulary"
          class="bg-green-600 text-white px-4 py-2 rounded disabled:opacity-50"
        >
          {{ isLoadingVocabulary ? 'Loading...' : 'Load Vocabulary' }}
        </button>
      </div>
    </div>

    <!-- Practice Section -->
    <div class="bg-white dark:bg-gray-800 rounded-lg p-4 shadow">
      <h3 class="text-lg font-semibold mb-4">Practice</h3>
      <div class="space-y-2">
        <div v-if="isLoadingStats" class="text-blue-600">Loading stats...</div>
        <div v-else-if="practiceStats" class="text-gray-600">
          Total sessions: {{ practiceStats.total_sessions }}, 
          Average score: {{ practiceStats.average_score }}%
        </div>
        
        <div v-if="statsError" class="text-red-600">
          Stats error: {{ statsError.message }}
        </div>
        
        <button 
          @click="loadStats" 
          :disabled="isLoadingStats"
          class="bg-purple-600 text-white px-4 py-2 rounded disabled:opacity-50"
        >
          {{ isLoadingStats ? 'Loading...' : 'Load Stats' }}
        </button>
      </div>
    </div>

    <!-- Error Display -->
    <div v-if="globalError" class="bg-red-100 dark:bg-red-900 border border-red-400 text-red-700 px-4 py-3 rounded">
      <strong>Global Error:</strong> {{ globalError.message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useAuth } from '../composables/useAuth';
import { useVocabulary } from '../composables/useVocabulary';
import { usePractice } from '../composables/usePractice';
import type { ApiError } from '../api/client';

// Composables
const { 
  user, 
  isLoadingUser, 
  login, 
  isLoggingIn, 
  loginError 
} = useAuth();

const { 
  words: vocabularyWords, 
  isLoading: isLoadingVocabulary, 
  error: vocabularyError,
  refetch: refetchVocabulary 
} = useVocabulary({ limit: 10 });

const { 
  stats: practiceStats, 
  isLoadingStats, 
  error: statsError,
  refetchStats 
} = usePractice();

// Local state
const globalError = ref<ApiError | null>(null);

// Methods
const handleLogin = async () => {
  try {
    globalError.value = null;
    await login({
      email: 'test@example.com',
      password: 'password123'
    });
  } catch (error) {
    globalError.value = error as ApiError;
  }
};

const loadVocabulary = async () => {
  try {
    globalError.value = null;
    await refetchVocabulary();
  } catch (error) {
    globalError.value = error as ApiError;
  }
};

const loadStats = async () => {
  try {
    globalError.value = null;
    await refetchStats();
  } catch (error) {
    globalError.value = error as ApiError;
  }
};

onMounted(() => {
  // Load initial data
  loadVocabulary();
  loadStats();
});
</script>
