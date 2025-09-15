<template>
  <form @submit.prevent="handleRegister" class="space-y-6">
    <div>
      <label for="name" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Full Name
      </label>
      <input
        id="name"
        v-model="formData.name"
        type="text"
        class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
        :class="errors.name ? 'border-red-500 focus:border-red-500 focus:ring-red-500' : 'border-gray-300 dark:border-gray-600 focus:border-blue-500'"
        placeholder="Enter your full name"
      />
      <p v-if="errors.name" class="mt-1 text-sm text-red-600 dark:text-red-400">
        {{ errors.name }}
      </p>
    </div>
    
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
        placeholder="Create a password"
      />
      <p v-if="errors.password" class="mt-1 text-sm text-red-600 dark:text-red-400">
        {{ errors.password }}
      </p>
    </div>
    
    <div>
      <label for="confirmPassword" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
        Confirm Password
      </label>
      <input
        id="confirmPassword"
        v-model="formData.confirmPassword"
        type="password"
        class="w-full px-3 py-2 border rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
        :class="errors.confirmPassword ? 'border-red-500 focus:border-red-500 focus:ring-red-500' : 'border-gray-300 dark:border-gray-600 focus:border-blue-500'"
        placeholder="Confirm your password"
      />
      <p v-if="errors.confirmPassword" class="mt-1 text-sm text-red-600 dark:text-red-400">
        {{ errors.confirmPassword }}
      </p>
    </div>
    
    <div v-if="registerError" class="text-red-600 dark:text-red-400 text-sm bg-red-50 dark:bg-red-900/20 p-3 rounded-md">
      {{ registerError.message || 'Registration failed. Please try again.' }}
    </div>
    
    <button
      type="submit"
      :disabled="isRegistering"
      class="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline transition-colors"
    >
      {{ isRegistering ? 'Creating Account...' : 'Create Account' }}
    </button>
    
    <div class="text-center">
      <p class="text-xs text-gray-500 dark:text-gray-400">
        By creating an account, you agree to our Terms of Service and Privacy Policy.
      </p>
    </div>
  </form>
</template>

<script setup lang="ts">
import { reactive, onMounted, ref } from 'vue';
import { registerSchema, type RegisterFormData } from '../schemas/auth';
import { z } from 'zod';

// Mock auth functions for SSR compatibility
const isRegistering = ref(false);
const registerError = ref(null);
let register: (data: RegisterFormData) => Promise<void> = async () => {};

onMounted(async () => {
  // Dynamically import auth composable only on client side
  const { useAuth } = await import('../composables/useAuth');
  const auth = useAuth();
  register = auth.register;
  isRegistering.value = auth.isRegistering.value;
  registerError.value = auth.registerError.value;
});

const formData = reactive<RegisterFormData>({
  name: '',
  email: '',
  password: '',
  confirmPassword: '',
});

const errors = reactive<Partial<Record<keyof RegisterFormData, string>>>({});

const validateForm = (): boolean => {
  try {
    registerSchema.parse(formData);
    Object.keys(errors).forEach(key => delete errors[key as keyof RegisterFormData]);
    return true;
  } catch (error) {
    if (error instanceof z.ZodError) {
      error.errors.forEach((err) => {
        if (err.path[0]) {
          errors[err.path[0] as keyof RegisterFormData] = err.message;
        }
      });
    }
    return false;
  }
};

const handleRegister = async () => {
  if (!validateForm()) return;
  
  try {
    await register(formData);
    // Redirect to dashboard on success
    if (typeof window !== 'undefined') {
      window.location.href = '/dashboard';
    }
  } catch (err) {
    // Error is handled by the useAuth composable
    console.error('Registration failed:', err);
  }
};
</script>
