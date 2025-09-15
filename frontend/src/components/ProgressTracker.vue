<template>
  <div class="space-y-6">
    <!-- Overview Stats -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <div class="flex items-center">
          <div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
            <Icon name="heroicons:chart-bar" class="w-6 h-6 text-blue-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Total Practice Time</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ totalPracticeTime }}</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <div class="flex items-center">
          <div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
            <Icon name="heroicons:arrow-trending-up" class="w-6 h-6 text-green-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Current Streak</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ currentStreak }} days</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <div class="flex items-center">
          <div class="p-2 bg-purple-100 dark:bg-purple-900 rounded-lg">
            <Icon name="heroicons:check-circle" class="w-6 h-6 text-purple-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Words Mastered</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ wordsMastered }}</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <div class="flex items-center">
          <div class="p-2 bg-yellow-100 dark:bg-yellow-900 rounded-lg">
            <Icon name="heroicons:star" class="w-6 h-6 text-yellow-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Average Score</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ averageScore }}%</p>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Charts Section -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Practice Time Chart -->
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Practice Time (Last 30 Days)
        </h3>
        <div class="h-64 flex items-end space-x-1">
          <div
            v-for="(day, index) in practiceTimeData"
            :key="index"
            class="bg-blue-500 rounded-t flex-1 relative group"
            :style="{ height: `${(day / Math.max(...practiceTimeData)) * 100}%` }"
          >
            <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 px-2 py-1 bg-gray-800 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity">
              {{ day }}min
            </div>
          </div>
        </div>
        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-2">
          <span>30 days ago</span>
          <span>Today</span>
        </div>
      </div>
      
      <!-- Score Improvement Chart -->
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          Score Improvement
        </h3>
        <div class="h-64 relative">
          <svg class="w-full h-full" viewBox="0 0 300 200">
            <polyline
              :points="scoreChartPoints"
              fill="none"
              stroke="#3B82F6"
              stroke-width="2"
            />
            <circle
              v-for="(point, index) in scoreChartData"
              :key="index"
              :cx="(index / (scoreChartData.length - 1)) * 300"
              :cy="200 - (point / 100) * 200"
              r="4"
              fill="#3B82F6"
            />
          </svg>
        </div>
        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400 mt-2">
          <span>Week 1</span>
          <span>Week 4</span>
        </div>
      </div>
    </div>
    
    <!-- Recent Practice Sessions -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        Recent Practice Sessions
      </h3>
      <div class="space-y-4">
        <div
          v-for="session in recentSessions"
          :key="session.id"
          class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg"
        >
          <div class="flex items-center space-x-4">
            <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center">
              <Icon name="heroicons:play" class="w-5 h-5 text-blue-600" />
            </div>
            <div>
              <p class="font-medium text-gray-900 dark:text-white">{{ session.word }}</p>
              <p class="text-sm text-gray-500 dark:text-gray-400">{{ session.date }}</p>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <div class="text-right">
              <p class="text-sm font-medium text-gray-900 dark:text-white">{{ session.score }}%</p>
              <p class="text-xs text-gray-500 dark:text-gray-400">{{ session.duration }}min</p>
            </div>
            <div class="w-16 bg-gray-200 dark:bg-gray-600 rounded-full h-2">
              <div
                class="bg-blue-600 h-2 rounded-full"
                :style="{ width: `${session.score}%` }"
              ></div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Phoneme Progress -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">
        Phoneme Mastery
      </h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div
          v-for="phoneme in phonemeProgress"
          :key="phoneme.symbol"
          class="text-center"
        >
          <div class="w-16 h-16 mx-auto mb-2 relative">
            <svg class="w-16 h-16 transform -rotate-90" viewBox="0 0 36 36">
              <path
                class="text-gray-200 dark:text-gray-700"
                stroke="currentColor"
                stroke-width="3"
                fill="none"
                d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              />
              <path
                class="text-blue-600"
                stroke="currentColor"
                stroke-width="3"
                fill="none"
                stroke-linecap="round"
                :stroke-dasharray="`${phoneme.mastery}, 100`"
                d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              />
            </svg>
            <div class="absolute inset-0 flex items-center justify-center">
              <span class="text-xs font-medium text-gray-900 dark:text-white">{{ phoneme.mastery }}%</span>
            </div>
          </div>
          <p class="text-sm font-mono text-gray-700 dark:text-gray-300">{{ phoneme.symbol }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';

const totalPracticeTime = ref('24h 30m');
const currentStreak = ref(12);
const wordsMastered = ref(156);
const averageScore = ref(78);

const practiceTimeData = ref([15, 20, 25, 18, 30, 22, 28, 35, 40, 32, 38, 45, 42, 38, 35, 40, 45, 50, 48, 52, 55, 58, 60, 55, 50, 45, 48, 52, 55, 60]);

const scoreChartData = ref([45, 52, 58, 62, 68, 72, 75, 78, 82, 85, 88, 90, 85, 88, 92, 95, 90, 88, 92, 95, 98, 95, 92, 95, 98, 100, 98, 95]);

const scoreChartPoints = computed(() => {
  return scoreChartData.value.map((score, index) => {
    const x = (index / (scoreChartData.value.length - 1)) * 300;
    const y = 200 - (score / 100) * 200;
    return `${x},${y}`;
  }).join(' ');
});

const recentSessions = ref([
  {
    id: 1,
    word: 'pronunciation',
    score: 85,
    duration: 3,
    date: '2 hours ago'
  },
  {
    id: 2,
    word: 'phonetic',
    score: 92,
    duration: 2,
    date: '1 day ago'
  },
  {
    id: 3,
    word: 'articulation',
    score: 78,
    duration: 4,
    date: '2 days ago'
  },
  {
    id: 4,
    word: 'rhythm',
    score: 88,
    duration: 2,
    date: '3 days ago'
  }
]);

const phonemeProgress = ref([
  { symbol: '/ə/', mastery: 85 },
  { symbol: '/ɪ/', mastery: 92 },
  { symbol: '/æ/', mastery: 78 },
  { symbol: '/ʌ/', mastery: 88 },
  { symbol: '/eɪ/', mastery: 75 },
  { symbol: '/aɪ/', mastery: 90 },
  { symbol: '/oʊ/', mastery: 82 },
  { symbol: '/aʊ/', mastery: 86 }
]);

onMounted(() => {
  // TODO: Load progress data from API
});
</script>
