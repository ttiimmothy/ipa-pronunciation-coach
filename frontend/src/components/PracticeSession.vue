<template>
  <div class="space-y-6">
    <!-- Practice Controls -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-xl font-semibold text-gray-900 dark:text-white">
          Practice Session
        </h2>
        <div class="flex items-center space-x-2">
          <span class="text-sm text-gray-500 dark:text-gray-400">Score:</span>
          <span class="text-2xl font-bold text-blue-600 dark:text-blue-400">
            {{ currentScore || '--' }}
          </span>
        </div>
      </div>
      
      <!-- Word Display -->
      <div class="text-center mb-6">
        <div class="text-4xl font-bold text-gray-900 dark:text-white mb-2">
          {{ currentWord?.word || 'Ready to practice?' }}
        </div>
        <div v-if="currentWord?.ipa" class="text-2xl text-blue-600 dark:text-blue-400 font-mono">
          /{{ currentWord.ipa }}/
        </div>
        <div v-if="currentWord?.definition" class="text-gray-600 dark:text-gray-300 mt-2">
          {{ currentWord.definition }}
        </div>
      </div>
      
      <!-- Recording Controls -->
      <div class="flex justify-center space-x-4 mb-6">
        <button
          @click="startRecording"
          :disabled="isRecording || isProcessing"
          class="bg-red-600 hover:bg-red-700 disabled:bg-red-400 text-white font-bold py-3 px-6 rounded-full transition-colors flex items-center space-x-2"
        >
          <Icon v-if="!isRecording" name="heroicons:microphone" class="w-5 h-5" />
          <Icon v-else name="heroicons:stop" class="w-5 h-5" />
          <span>{{ isRecording ? 'Stop' : 'Record' }}</span>
        </button>
        
        <button
          @click="playReference"
          :disabled="!currentWord || isProcessing"
          class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-3 px-6 rounded-full transition-colors flex items-center space-x-2"
        >
          <Icon name="heroicons:play" class="w-5 h-5" />
          <span>Play Reference</span>
        </button>
      </div>
      
      <!-- Progress Bar -->
      <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2 mb-4">
        <div 
          class="bg-blue-600 h-2 rounded-full transition-all duration-300"
          :style="{ width: `${(currentWordIndex / totalWords) * 100}%` }"
        ></div>
      </div>
      
      <div class="flex justify-between text-sm text-gray-500 dark:text-gray-400">
        <span>Word {{ currentWordIndex + 1 }} of {{ totalWords }}</span>
        <span>{{ Math.round((currentWordIndex / totalWords) * 100) }}% Complete</span>
      </div>
    </div>
    
    <!-- Feedback Section -->
    <div v-if="feedback" class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        Pronunciation Feedback
      </h3>
      
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <span class="text-gray-700 dark:text-gray-300">Overall Score</span>
          <div class="flex items-center space-x-2">
            <div class="w-32 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
              <div 
                class="bg-green-500 h-2 rounded-full transition-all duration-300"
                :style="{ width: `${feedback.overallScore}%` }"
              ></div>
            </div>
            <span class="text-sm font-medium text-gray-900 dark:text-white">
              {{ feedback.overallScore }}%
            </span>
          </div>
        </div>
        
        <div v-if="feedback.phonemeScores" class="space-y-2">
          <h4 class="font-medium text-gray-900 dark:text-white">Phoneme Breakdown</h4>
          <div class="grid grid-cols-2 gap-2">
            <div 
              v-for="(score, phoneme) in feedback.phonemeScores" 
              :key="phoneme"
              class="flex items-center justify-between p-2 bg-gray-50 dark:bg-gray-700 rounded"
            >
              <span class="text-sm font-mono text-gray-700 dark:text-gray-300">{{ phoneme }}</span>
              <span class="text-sm font-medium" :class="score > 80 ? 'text-green-600' : score > 60 ? 'text-yellow-600' : 'text-red-600'">
                {{ score }}%
              </span>
            </div>
          </div>
        </div>
        
        <div v-if="feedback.suggestions" class="mt-4">
          <h4 class="font-medium text-gray-900 dark:text-white mb-2">Suggestions</h4>
          <ul class="space-y-1">
            <li 
              v-for="suggestion in feedback.suggestions" 
              :key="suggestion"
              class="text-sm text-gray-600 dark:text-gray-300 flex items-start space-x-2"
            >
              <svg class="w-4 h-4 text-blue-500 mt-0.5 flex-shrink-0" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
              </svg>
              <span>{{ suggestion }}</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
    
    <!-- Navigation -->
    <div class="flex justify-between">
      <button
        @click="previousWord"
        :disabled="currentWordIndex === 0"
        class="bg-gray-600 hover:bg-gray-700 disabled:bg-gray-400 text-white font-bold py-2 px-4 rounded transition-colors"
      >
        Previous
      </button>
      
      <button
        @click="nextWord"
        :disabled="currentWordIndex >= totalWords - 1"
        class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded transition-colors"
      >
        Next Word
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import Icon from './Icon.vue';

const currentWord = ref<{word: string, ipa: string, definition: string} | null>(null);
const currentWordIndex = ref(0);
const totalWords = ref(10);
const isRecording = ref(false);
const isProcessing = ref(false);
const currentScore = ref<number | null>(null);
const feedback = ref<{
  overallScore: number;
  phonemeScores: Record<string, number>;
  suggestions: string[];
} | null>(null);

// Mock data for demonstration
const practiceWords = [
  {
    word: 'pronunciation',
    ipa: 'prəˌnʌnsiˈeɪʃən',
    definition: 'The way in which a word is pronounced'
  },
  {
    word: 'phonetic',
    ipa: 'fəˈnetɪk',
    definition: 'Relating to speech sounds'
  },
  {
    word: 'articulation',
    ipa: 'ɑrˌtɪkjəˈleɪʃən',
    definition: 'The formation of clear and distinct sounds'
  }
];

const startRecording = () => {
  isRecording.value = true;
  // TODO: Implement actual recording logic
  setTimeout(() => {
    isRecording.value = false;
    processRecording();
  }, 3000);
};

const processRecording = () => {
  isProcessing.value = true;
  // TODO: Implement actual processing logic
  setTimeout(() => {
    currentScore.value = Math.floor(Math.random() * 40) + 60; // Mock score
    feedback.value = {
      overallScore: currentScore.value,
      phonemeScores: {
        'prə': 85,
        'ˌnʌn': 70,
        'si': 90,
        'ˈeɪ': 65,
        'ʃən': 75
      },
      suggestions: [
        'Focus on the stress pattern in the second syllable',
        'The "n" sound should be more nasal',
        'Try to make the "ʃ" sound more distinct'
      ]
    };
    isProcessing.value = false;
  }, 2000);
};

const playReference = () => {
  // TODO: Implement actual audio playback
  console.log('Playing reference audio for:', currentWord.value?.word);
};

const nextWord = () => {
  if (currentWordIndex.value < totalWords.value - 1) {
    currentWordIndex.value++;
    loadCurrentWord();
  }
};

const previousWord = () => {
  if (currentWordIndex.value > 0) {
    currentWordIndex.value--;
    loadCurrentWord();
  }
};

const loadCurrentWord = () => {
  currentWord.value = practiceWords[currentWordIndex.value % practiceWords.length];
  currentScore.value = null;
  feedback.value = null;
};

onMounted(() => {
  loadCurrentWord();
});
</script>
