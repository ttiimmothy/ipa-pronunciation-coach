<template>
  <div class="space-y-6">
    <!-- Profile Settings -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Profile Information
      </h2>
      
      <form @submit.prevent="updateProfile" class="space-y-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label for="firstName" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              First Name
            </label>
            <input
              id="firstName"
              v-model="profile.firstName"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          
          <div>
            <label for="lastName" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Last Name
            </label>
            <input
              id="lastName"
              v-model="profile.lastName"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
        </div>
        
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Email
          </label>
          <input
            id="email"
            v-model="profile.email"
            type="email"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        
        <div>
          <label for="nativeLanguage" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
            Native Language
          </label>
          <select
            id="nativeLanguage"
            v-model="profile.nativeLanguage"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="en">English</option>
            <option value="es">Spanish</option>
            <option value="fr">French</option>
            <option value="de">German</option>
            <option value="it">Italian</option>
            <option value="pt">Portuguese</option>
            <option value="ru">Russian</option>
            <option value="ja">Japanese</option>
            <option value="ko">Korean</option>
            <option value="zh">Chinese</option>
            <option value="other">Other</option>
          </select>
        </div>
        
        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="profileLoading"
            class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            {{ profileLoading ? 'Saving...' : 'Save Changes' }}
          </button>
        </div>
      </form>
    </div>
    
    <!-- Learning Preferences -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Learning Preferences
      </h2>
      
      <form @submit.prevent="updatePreferences" class="space-y-6">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Difficulty Level
          </label>
          <div class="space-y-2">
            <label v-for="level in difficultyLevels" :key="level.value" class="flex items-center">
              <input
                v-model="preferences.difficulty"
                :value="level.value"
                type="radio"
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">{{ level.label }}</span>
            </label>
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Practice Session Length
          </label>
          <select
            v-model="preferences.sessionLength"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="5">5 minutes</option>
            <option value="10">10 minutes</option>
            <option value="15">15 minutes</option>
            <option value="20">20 minutes</option>
            <option value="30">30 minutes</option>
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Daily Practice Goal
          </label>
          <div class="flex items-center space-x-4">
            <input
              v-model="preferences.dailyGoal"
              type="range"
              min="5"
              max="60"
              step="5"
              class="flex-1"
            />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ preferences.dailyGoal }} minutes
            </span>
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Notification Preferences
          </label>
          <div class="space-y-2">
            <label class="flex items-center">
              <input
                v-model="preferences.notifications.dailyReminder"
                type="checkbox"
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Daily practice reminder</span>
            </label>
            <label class="flex items-center">
              <input
                v-model="preferences.notifications.weeklyReport"
                type="checkbox"
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Weekly progress report</span>
            </label>
            <label class="flex items-center">
              <input
                v-model="preferences.notifications.achievements"
                type="checkbox"
                class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
              />
              <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Achievement notifications</span>
            </label>
          </div>
        </div>
        
        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="preferencesLoading"
            class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            {{ preferencesLoading ? 'Saving...' : 'Save Preferences' }}
          </button>
        </div>
      </form>
    </div>
    
    <!-- Audio Settings -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Audio Settings
      </h2>
      
      <form @submit.prevent="updateAudioSettings" class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Microphone Sensitivity
          </label>
          <div class="flex items-center space-x-4">
            <input
              v-model="audioSettings.sensitivity"
              type="range"
              min="0"
              max="100"
              class="flex-1"
            />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
              {{ audioSettings.sensitivity }}%
            </span>
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
            Playback Speed
          </label>
          <select
            v-model="audioSettings.playbackSpeed"
            class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="0.5">0.5x (Slow)</option>
            <option value="0.75">0.75x</option>
            <option value="1">1x (Normal)</option>
            <option value="1.25">1.25x</option>
            <option value="1.5">1.5x (Fast)</option>
          </select>
        </div>
        
        <div>
          <label class="flex items-center">
            <input
              v-model="audioSettings.autoPlay"
              type="checkbox"
              class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <span class="ml-2 text-sm text-gray-700 dark:text-gray-300">Auto-play reference audio</span>
          </label>
        </div>
        
        <div class="flex justify-end">
          <button
            type="submit"
            :disabled="audioLoading"
            class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            {{ audioLoading ? 'Saving...' : 'Save Audio Settings' }}
          </button>
        </div>
      </form>
    </div>
    
    <!-- Account Actions -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white mb-6">
        Account Actions
      </h2>
      
      <div class="space-y-4">
        <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <div>
            <h3 class="font-medium text-gray-900 dark:text-white">Change Password</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Update your account password</p>
          </div>
          <button
            @click="showChangePassword = true"
            class="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            Change
          </button>
        </div>
        
        <div class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <div>
            <h3 class="font-medium text-gray-900 dark:text-white">Export Data</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Download your practice data</p>
          </div>
          <button
            @click="exportData"
            class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            Export
          </button>
        </div>
        
        <div class="flex items-center justify-between p-4 bg-red-50 dark:bg-red-900/20 rounded-lg">
          <div>
            <h3 class="font-medium text-red-900 dark:text-red-300">Delete Account</h3>
            <p class="text-sm text-red-600 dark:text-red-400">Permanently delete your account and data</p>
          </div>
          <button
            @click="showDeleteAccount = true"
            class="bg-red-600 hover:bg-red-700 text-white font-bold py-2 px-4 rounded transition-colors"
          >
            Delete
          </button>
        </div>
      </div>
    </div>
    
    <!-- Change Password Modal -->
    <div v-if="showChangePassword" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div class="bg-white dark:bg-gray-800 rounded-lg p-6 w-full max-w-md">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Change Password</h3>
        <form @submit.prevent="changePassword" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Current Password
            </label>
            <input
              v-model="passwordForm.current"
              type="password"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              New Password
            </label>
            <input
              v-model="passwordForm.new"
              type="password"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Confirm New Password
            </label>
            <input
              v-model="passwordForm.confirm"
              type="password"
              class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
            />
          </div>
          <div class="flex justify-end space-x-2">
            <button
              type="button"
              @click="showChangePassword = false"
              class="bg-gray-600 hover:bg-gray-700 text-white font-bold py-2 px-4 rounded transition-colors"
            >
              Cancel
            </button>
            <button
              type="submit"
              :disabled="passwordLoading"
              class="bg-blue-600 hover:bg-blue-700 disabled:bg-blue-400 text-white font-bold py-2 px-4 rounded transition-colors"
            >
              {{ passwordLoading ? 'Changing...' : 'Change Password' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

const profile = ref({
  firstName: 'John',
  lastName: 'Doe',
  email: 'john.doe@example.com',
  nativeLanguage: 'en'
});

const preferences = ref({
  difficulty: 'intermediate',
  sessionLength: 15,
  dailyGoal: 20,
  notifications: {
    dailyReminder: true,
    weeklyReport: true,
    achievements: true
  }
});

const audioSettings = ref({
  sensitivity: 75,
  playbackSpeed: 1,
  autoPlay: true
});

const difficultyLevels = [
  { value: 'beginner', label: 'Beginner' },
  { value: 'intermediate', label: 'Intermediate' },
  { value: 'advanced', label: 'Advanced' }
];

const showChangePassword = ref(false);
const showDeleteAccount = ref(false);
const passwordForm = ref({
  current: '',
  new: '',
  confirm: ''
});

const profileLoading = ref(false);
const preferencesLoading = ref(false);
const audioLoading = ref(false);
const passwordLoading = ref(false);

const updateProfile = async () => {
  profileLoading.value = true;
  // TODO: Implement profile update
  setTimeout(() => {
    profileLoading.value = false;
  }, 1000);
};

const updatePreferences = async () => {
  preferencesLoading.value = true;
  // TODO: Implement preferences update
  setTimeout(() => {
    preferencesLoading.value = false;
  }, 1000);
};

const updateAudioSettings = async () => {
  audioLoading.value = true;
  // TODO: Implement audio settings update
  setTimeout(() => {
    audioLoading.value = false;
  }, 1000);
};

const changePassword = async () => {
  if (passwordForm.value.new !== passwordForm.value.confirm) {
    alert('Passwords do not match');
    return;
  }
  
  passwordLoading.value = true;
  // TODO: Implement password change
  setTimeout(() => {
    passwordLoading.value = false;
    showChangePassword.value = false;
    passwordForm.value = { current: '', new: '', confirm: '' };
  }, 1000);
};

const exportData = () => {
  // TODO: Implement data export
  console.log('Exporting data...');
};

onMounted(() => {
  // TODO: Load user settings from API
});
</script>
