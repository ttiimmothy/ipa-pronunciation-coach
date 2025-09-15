<template>
  <form @submit.prevent="handleLogin" class="space-y-4">
    <div>
      <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Email
      </label>
      <input
        id="email"
        v-model="formData.email"
        type="email"
        class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
        :class="errors.email ? 'border-red-500 focus:border-red-500 focus:ring-red-500' : 'border-gray-300 dark:border-gray-600 focus:border-blue-500'"
        placeholder="Enter your email"
      />
      <p v-if="errors.email" class="mt-1 text-sm text-red-600 dark:text-red-400">
        {{ errors.email }}
      </p>
    </div>
    
    <div>
      <label for="password" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Password
      </label>
      <input
        id="password"
        v-model="formData.password"
        type="password"
        class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
        :class="errors.password ? 'border-red-500 focus:border-red-500 focus:ring-red-500' : 'border-gray-300 dark:border-gray-600 focus:border-blue-500'"
        placeholder="Enter your password"
      />
      <p v-if="errors.password" class="mt-1 text-sm text-red-600 dark:text-red-400">
        {{ errors.password }}
      </p>
    </div>
    
    <div v-if="loginError" class="text-red-600 dark:text-red-400 text-sm bg-red-50 dark:bg-red-900/20 p-3 rounded-md">
      {{ loginError.message || 'Login failed. Please try again.' }}
    </div>
    
    <button
      type="submit"
      :disabled="isLoggingIn"
      class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-colors"
    >
      {{ isLoggingIn ? 'Logging in...' : 'Login' }}
    </button>
    
    <div class="text-center">
      <a href="/register" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 text-sm">
        Don't have an account? Register
      </a>
    </div>
  </form>
</template>

<script setup lang="ts">
import { reactive, onMounted, ref } from 'vue';
import { loginSchema, type LoginFormData } from '../schemas/auth';
import { z } from 'zod';

// Mock auth functions for SSR compatibility
const isLoggingIn = ref(false);
const loginError = ref(null);
let login: (data: LoginFormData) => void = () => {};

onMounted(async () => {
  // Dynamically import auth composable only on client side
  const { useAuth } = await import('../composables/useAuth');
  const {login: authLogin, isLoggingIn: authIsLoggingIn, loginError: authLoginError} = useAuth();
  login = authLogin
  isLoggingIn.value = authIsLoggingIn;
  loginError.value = authLoginError;
});

const formData = reactive<LoginFormData>({
  email: '',
  password: '',
});

const errors = reactive<Partial<Record<keyof LoginFormData, string>>>({});

const validateForm = (): boolean => {
  try {
    loginSchema.parse(formData);
    Object.keys(errors).forEach(key => delete errors[key as keyof LoginFormData]);
    return true;
  } catch (error) {
    if (error instanceof z.ZodError) {
      error.errors.forEach((err) => {
        if (err.path[0]) {
          errors[err.path[0] as keyof LoginFormData] = err.message;
        }
      });
    }
    return false;
  }
};

const handleLogin = async () => {
  if (!validateForm()) return;
  
  try {
    await login(formData);
    // Redirect to dashboard on success
    if (typeof window !== 'undefined') {
      window.location.href = '/dashboard';
    }
  } catch (err) {
    // Error is handled by the useAuth composable
    console.error('Login failed:', err);
  }
};
</script>
