<template>
  <div class="bg-white dark:bg-gray-800 shadow-md rounded-lg p-6 hover:shadow-lg transition-shadow">
    <div class="flex items-start justify-between mb-4">
      <div>
        <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
          {{ word.text }}
        </h3>
        <p v-if="word.pos" class="text-sm text-gray-500 dark:text-gray-400 capitalize">
          {{ word.pos }}
        </p>
      </div>
      <div class="flex items-center space-x-2">
        <span class="text-sm text-gray-500 dark:text-gray-400">
          Difficulty: {{ word.difficulty }}/5
        </span>
        <div class="flex space-x-1">
          <span
            v-for="i in 5"
            :key="i"
            class="w-2 h-2 rounded-full"
            :class="i <= word.difficulty ? 'bg-yellow-400' : 'bg-gray-300 dark:bg-gray-600'"
          />
        </div>
      </div>
    </div>
    
    <div class="space-y-3">
      <div v-for="variant in word.variants" :key="variant.id" class="border-l-4 border-blue-500 pl-4">
        <div class="flex items-center space-x-2 mb-2">
          <IpaBadge :ipa="variant.ipa" />
          <span class="text-sm font-medium text-gray-600 dark:text-gray-300">
            {{ variant.dialect }}
          </span>
        </div>
        
        <div v-if="variant.audio_url" class="flex items-center space-x-2">
          <button
            @click="playAudio(variant.audio_url)"
            class="flex items-center space-x-1 text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"
          >
            <Icon name="heroicons:play" class="w-4 h-4" />
            <span class="text-sm">Play Audio</span>
          </button>
        </div>
      </div>
    </div>
    
    <div class="mt-4 flex space-x-2">
      <button
        @click="$emit('practice', word)"
        class="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded text-sm font-medium transition-colors"
      >
        Practice
      </button>
      <button
        @click="$emit('view-details', word)"
        class="bg-gray-600 hover:bg-gray-700 text-white px-4 py-2 rounded text-sm font-medium transition-colors"
      >
        View Details
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import IpaBadge from './IpaBadge.vue';
import type { WordWithVariants } from '@ipa-coach/shared';

defineProps<{
  word: WordWithVariants;
}>();

defineEmits<{
  practice: [word: WordWithVariants];
  'view-details': [word: WordWithVariants];
}>();

const playAudio = (audioUrl: string) => {
  const audio = new Audio(audioUrl);
  audio.play().catch(console.error);
};
</script>
