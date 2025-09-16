<template>
  <button
    @click="toggleDarkMode"
    class="relative w-16 h-8 rounded-full bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 transition-all duration-300 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
    :title="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
  >
    <!-- Sliding toggle background -->
    <div 
      class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full shadow-md transform transition-transform duration-300 ease-in-out flex items-center justify-center"
      :class="{ 'translate-x-8': isDark }"
    >
      <!-- Moon icon (light mode) -->
      <svg 
        v-if="!isDark" 
        class="w-4 h-4 text-gray-600" 
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
      </svg>
      <!-- Sun icon (dark mode) -->
      <svg 
        v-else 
        class="w-4 h-4 text-orange-600" 
        fill="none" 
        stroke="currentColor" 
        viewBox="0 0 24 24"
      >
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
      </svg>
    </div>
  </button>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

const isDark = ref(false);

const toggleDarkMode = () => {
  isDark.value = !isDark.value;
  updateTheme();
};

const updateTheme = () => {
  if (isDark.value) {
    document.documentElement.classList.add('dark');
    localStorage.setItem('theme', 'dark');
  } else {
    document.documentElement.classList.remove('dark');
    localStorage.setItem('theme', 'light');
  }
};

onMounted(() => {
  // Check for saved theme preference or default to light mode
  const savedTheme = localStorage.getItem('theme');
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  
  isDark.value = savedTheme === 'dark';
  updateTheme();
});
</script>
