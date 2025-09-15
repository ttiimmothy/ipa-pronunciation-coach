<template>
  <div class="space-y-6">
    <!-- Search and Filters -->
    <div class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6">
      <div class="flex flex-col md:flex-row gap-4 mb-4">
        <div class="flex-1">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Search words, IPA, or definitions..."
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>
        <div class="flex gap-2">
          <select
            v-model="selectedDifficulty"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="">All Difficulties</option>
            <option value="beginner">Beginner</option>
            <option value="intermediate">Intermediate</option>
            <option value="advanced">Advanced</option>
          </select>
          <select
            v-model="selectedCategory"
            class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          >
            <option value="">All Categories</option>
            <option value="common">Common Words</option>
            <option value="academic">Academic</option>
            <option value="business">Business</option>
            <option value="technical">Technical</option>
          </select>
        </div>
      </div>
      
      <div class="flex flex-wrap gap-2">
        <button
          v-for="tag in popularTags"
          :key="tag"
          @click="toggleTag(tag)"
          :class="[
            'px-3 py-1 rounded-full text-sm transition-colors',
            selectedTags.includes(tag)
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          ]"
        >
          {{ tag }}
        </button>
      </div>
    </div>
    
    <!-- Vocabulary Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="word in filteredWords"
        :key="word.id"
        class="bg-white dark:bg-gray-800 shadow-lg rounded-lg p-6 card-hover"
      >
        <div class="flex justify-between items-start mb-3">
          <h3 class="text-xl font-semibold text-gray-900 dark:text-white">
            {{ word.word }}
          </h3>
          <div class="flex items-center space-x-1">
            <span
              :class="[
                'px-2 py-1 rounded-full text-xs font-medium',
                getDifficultyClass(word.difficulty)
              ]"
            >
              {{ word.difficulty }}
            </span>
          </div>
        </div>
        
        <div class="mb-3">
          <div class="text-lg text-blue-600 dark:text-blue-400 font-mono">
            /{{ word.ipa }}/
          </div>
          <div class="text-sm text-gray-600 dark:text-gray-300">
            {{ word.definition }}
          </div>
        </div>
        
        <div class="flex flex-wrap gap-1 mb-4">
          <span
            v-for="tag in word.tags"
            :key="tag"
            class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 text-xs rounded"
          >
            {{ tag }}
          </span>
        </div>
        
        <div class="flex justify-between items-center">
          <div class="flex items-center space-x-2">
            <button
              @click="playAudio(word)"
              class="p-2 text-gray-500 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
            >
              <Icon name="heroicons:play" class="w-5 h-5" />
            </button>
            <button
              @click="addToPractice(word)"
              class="p-2 text-gray-500 hover:text-green-600 dark:hover:text-green-400 transition-colors"
            >
              <Icon name="heroicons:plus" class="w-5 h-5" />
            </button>
          </div>
          
          <div class="flex items-center space-x-1">
            <span class="text-sm text-gray-500 dark:text-gray-400">
              {{ word.practiceCount || 0 }} practices
            </span>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Pagination -->
    <div class="flex justify-center">
      <div class="flex space-x-2">
        <button
          @click="previousPage"
          :disabled="currentPage === 1"
          class="px-3 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Previous
        </button>
        <button
          v-for="page in visiblePages"
          :key="page"
          @click="goToPage(page)"
          :class="[
            'px-3 py-2 rounded',
            page === currentPage
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600'
          ]"
        >
          {{ page }}
        </button>
        <button
          @click="nextPage"
          :disabled="currentPage === totalPages"
          class="px-3 py-2 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Next
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';

const searchQuery = ref('');
const selectedDifficulty = ref('');
const selectedCategory = ref('');
const selectedTags = ref([]);
const currentPage = ref(1);
const itemsPerPage = 12;

const popularTags = ['vowels', 'consonants', 'stress', 'rhythm', 'intonation'];

// Mock data
const words = ref([
  {
    id: 1,
    word: 'pronunciation',
    ipa: 'prəˌnʌnsiˈeɪʃən',
    definition: 'The way in which a word is pronounced',
    difficulty: 'intermediate',
    category: 'common',
    tags: ['stress', 'syllables'],
    practiceCount: 5
  },
  {
    id: 2,
    word: 'phonetic',
    ipa: 'fəˈnetɪk',
    definition: 'Relating to speech sounds',
    difficulty: 'beginner',
    category: 'academic',
    tags: ['vowels', 'consonants'],
    practiceCount: 3
  },
  {
    id: 3,
    word: 'articulation',
    ipa: 'ɑrˌtɪkjəˈleɪʃən',
    definition: 'The formation of clear and distinct sounds',
    difficulty: 'advanced',
    category: 'academic',
    tags: ['consonants', 'stress'],
    practiceCount: 8
  },
  {
    id: 4,
    word: 'rhythm',
    ipa: 'ˈrɪðəm',
    definition: 'A strong, regular, repeated pattern of movement or sound',
    difficulty: 'intermediate',
    category: 'common',
    tags: ['stress', 'rhythm'],
    practiceCount: 2
  },
  {
    id: 5,
    word: 'intonation',
    ipa: 'ˌɪntəˈneɪʃən',
    definition: 'The rise and fall of the voice in speaking',
    difficulty: 'advanced',
    category: 'academic',
    tags: ['intonation', 'pitch'],
    practiceCount: 6
  },
  {
    id: 6,
    word: 'syllable',
    ipa: 'ˈsɪləbəl',
    definition: 'A unit of pronunciation having one vowel sound',
    difficulty: 'beginner',
    category: 'common',
    tags: ['syllables', 'vowels'],
    practiceCount: 4
  }
]);

const filteredWords = computed(() => {
  let filtered = words.value;
  
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase();
    filtered = filtered.filter(word => 
      word.word.toLowerCase().includes(query) ||
      word.ipa.toLowerCase().includes(query) ||
      word.definition.toLowerCase().includes(query)
    );
  }
  
  if (selectedDifficulty.value) {
    filtered = filtered.filter(word => word.difficulty === selectedDifficulty.value);
  }
  
  if (selectedCategory.value) {
    filtered = filtered.filter(word => word.category === selectedCategory.value);
  }
  
  if (selectedTags.value.length > 0) {
    filtered = filtered.filter(word => 
      selectedTags.value.some(tag => word.tags.includes(tag))
    );
  }
  
  const start = (currentPage.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return filtered.slice(start, end);
});

const totalPages = computed(() => {
  return Math.ceil(words.value.length / itemsPerPage);
});

const visiblePages = computed(() => {
  const pages = [];
  const start = Math.max(1, currentPage.value - 2);
  const end = Math.min(totalPages.value, start + 4);
  
  for (let i = start; i <= end; i++) {
    pages.push(i);
  }
  return pages;
});

const getDifficultyClass = (difficulty) => {
  switch (difficulty) {
    case 'beginner':
      return 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200';
    case 'intermediate':
      return 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200';
    case 'advanced':
      return 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200';
    default:
      return 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200';
  }
};

const toggleTag = (tag) => {
  const index = selectedTags.value.indexOf(tag);
  if (index > -1) {
    selectedTags.value.splice(index, 1);
  } else {
    selectedTags.value.push(tag);
  }
};

const playAudio = (word) => {
  // TODO: Implement audio playback
  console.log('Playing audio for:', word.word);
};

const addToPractice = (word) => {
  // TODO: Add word to practice list
  console.log('Adding to practice:', word.word);
};

const previousPage = () => {
  if (currentPage.value > 1) {
    currentPage.value--;
  }
};

const nextPage = () => {
  if (currentPage.value < totalPages.value) {
    currentPage.value++;
  }
};

const goToPage = (page) => {
  currentPage.value = page;
};

onMounted(() => {
  // TODO: Load vocabulary from API
});
</script>
