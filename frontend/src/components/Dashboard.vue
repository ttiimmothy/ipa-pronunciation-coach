<template>
  <div class="space-y-6">
    <!-- Welcome Section -->
    <div class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg p-6 text-white">
      <h1 class="text-3xl font-bold mb-2">
        Welcome back, {{ user?.name || 'User' }}!
      </h1>
      <p class="text-blue-100 text-lg">
        Ready to practice your pronunciation? Let's make today count!
      </p>
    </div>
    
    <!-- Stats Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 card-hover">
        <div class="flex items-center">
          <div class="p-2 bg-blue-100 dark:bg-blue-900 rounded-lg">
            <Icon name="heroicons:clock" class="w-6 h-6 text-blue-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Today's Practice</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ formatTime(dailyProgress?.activeMs || 0) }}</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 card-hover">
        <div class="flex items-center">
          <div class="p-2 bg-green-100 dark:bg-green-900 rounded-lg">
            <Icon name="heroicons:chart-bar" class="w-6 h-6 text-green-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Current Streak</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ streak || 0 }} days</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 card-hover">
        <div class="flex items-center">
          <div class="p-2 bg-purple-100 dark:bg-purple-900 rounded-lg">
            <Icon name="heroicons:check-circle" class="w-6 h-6 text-purple-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Words Mastered</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ wordsLearned || 0 }}</p>
          </div>
        </div>
      </div>
      
      <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 card-hover">
        <div class="flex items-center">
          <div class="p-2 bg-yellow-100 dark:bg-yellow-900 rounded-lg">
            <Icon name="heroicons:star" class="w-6 h-6 text-yellow-600" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600 dark:text-gray-400">Average Score</p>
            <p class="text-2xl font-semibold text-gray-900 dark:text-white">{{ averageScore || 0 }}%</p>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Quick Actions -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Quick Actions
      </h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <a
          href="/practice"
          class="bg-gradient-to-r from-green-500 to-green-600 hover:from-green-600 hover:to-green-700 text-white font-bold py-4 px-6 rounded-lg text-center transition-all transform hover:scale-105 flex items-center justify-center space-x-2"
        >
          <Icon name="heroicons:play" class="w-5 h-5" />
          <span>Start Practice</span>
        </a>
        
        <a
          href="/vocab"
          class="bg-gradient-to-r from-blue-500 to-blue-600 hover:from-blue-600 hover:to-blue-700 text-white font-bold py-4 px-6 rounded-lg text-center transition-all transform hover:scale-105 flex items-center justify-center space-x-2"
        >
          <Icon name="heroicons:book-open" class="w-5 h-5" />
          <span>Browse Vocabulary</span>
        </a>
        
        <a
          href="/progress"
          class="bg-gradient-to-r from-purple-500 to-purple-600 hover:from-purple-600 hover:to-purple-700 text-white font-bold py-4 px-6 rounded-lg text-center transition-all transform hover:scale-105 flex items-center justify-center space-x-2"
        >
          <Icon name="heroicons:chart-bar-square" class="w-5 h-5" />
          <span>View Progress</span>
        </a>
        
        <a
          href="/settings"
          class="bg-gradient-to-r from-gray-500 to-gray-600 hover:from-gray-600 hover:to-gray-700 text-white font-bold py-4 px-6 rounded-lg text-center transition-all transform hover:scale-105 flex items-center justify-center space-x-2"
        >
          <Icon name="heroicons:cog-6-tooth" class="w-5 h-5" />
          <span>Settings</span>
        </a>
      </div>
    </div>
    
    <!-- Recent Activity -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Recent Activity
      </h3>
      <div class="space-y-4">
        <div
          v-for="activity in recentActivity"
          :key="activity.id"
          class="flex items-center space-x-4 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg"
        >
          <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-full flex items-center justify-center">
            <Icon name="heroicons:play" class="w-5 h-5 text-blue-600" />
          </div>
          <div class="flex-1">
            <p class="font-medium text-gray-900 dark:text-white">{{ activity.action }}</p>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ activity.time }}</p>
          </div>
          <div class="text-right">
            <span class="text-sm font-medium text-gray-900 dark:text-white">{{ activity.score }}%</span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Daily Goal Progress -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h3 class="text-xl font-semibold text-gray-900 dark:text-white mb-4">
        Daily Goal Progress
      </h3>
      <div class="flex items-center space-x-4">
        <div class="flex-1">
          <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3">
            <div 
              class="bg-gradient-to-r from-green-500 to-green-600 h-3 rounded-full transition-all duration-300"
              :style="{ width: `${Math.min((dailyProgress?.activeMs || 0) / (dailyGoal * 60 * 1000) * 100, 100)}%` }"
            ></div>
          </div>
        </div>
        <div class="text-sm font-medium text-gray-700 dark:text-gray-300">
          {{ Math.round((dailyProgress?.activeMs || 0) / (dailyGoal * 60 * 1000) * 100) }}%
        </div>
      </div>
      <p class="text-sm text-gray-500 dark:text-gray-400 mt-2">
        {{ formatTime(dailyProgress?.activeMs || 0) }} of {{ dailyGoal }} minutes today
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

// Mock data for SSR compatibility
const user = ref({
  name: 'Demo User',
  email: 'demo@example.com'
});
const dailyProgress = ref({ activeMs: 1800000 }); // 30 minutes in milliseconds
const streak = ref(12);
const wordsLearned = ref(156);
const averageScore = ref(78);
const dailyGoal = ref(30); // 30 minutes

const recentActivity = ref([
  {
    id: 1,
    action: 'Practiced "pronunciation"',
    time: '2 hours ago',
    score: 85
  },
  {
    id: 2,
    action: 'Mastered "phonetic"',
    time: '1 day ago',
    score: 92
  },
  {
    id: 3,
    action: 'Practiced "articulation"',
    time: '2 days ago',
    score: 78
  },
  {
    id: 4,
    action: 'Completed daily goal',
    time: '3 days ago',
    score: 100
  }
]);

const formatTime = (milliseconds: number) => {
  const minutes = Math.floor(milliseconds / 60000);
  const hours = Math.floor(minutes / 60);
  const remainingMinutes = minutes % 60;
  
  if (hours > 0) {
    return `${hours}h ${remainingMinutes}m`;
  }
  return `${minutes}m`;
};

onMounted(async () => {
  // Load user data and dashboard stats
  // TODO: Load dashboard data from API
});
</script>
