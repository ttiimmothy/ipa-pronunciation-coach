<template>
  <i :class="[iconClass, props.class]" />
</template>

<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  name: string;
  class?: string;
}>();

const iconClass = computed(() => {
  // Map icon names to their CSS classes
  const iconMap: Record<string, string> = {
    'heroicons:sun': 'i-heroicons-sun',
    'heroicons:moon': 'i-heroicons-moon',
    'heroicons:clock': 'i-heroicons-clock',
    'heroicons:chart-bar': 'i-heroicons-chart-bar',
    'heroicons:check-circle': 'i-heroicons-check-circle',
    'heroicons:star': 'i-heroicons-star',
    'heroicons:play': 'i-heroicons-play',
    'heroicons:book-open': 'i-heroicons-book-open',
    'heroicons:chart-bar-square': 'i-heroicons-chart-bar-square',
    'heroicons:cog-6-tooth': 'i-heroicons-cog-6-tooth',
    'heroicons:arrow-trending-up': 'i-heroicons-arrow-trending-up',
    'heroicons:microphone': 'i-heroicons-microphone',
    'heroicons:stop': 'i-heroicons-stop',
    'heroicons:plus': 'i-heroicons-plus',
  };
  
  return iconMap[props.name] || props.name;
});
</script>
