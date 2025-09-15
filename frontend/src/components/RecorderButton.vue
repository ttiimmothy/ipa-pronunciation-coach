<template>
  <div class="flex flex-col items-center space-y-4">
    <button
      @click="toggleRecording"
      :disabled="disabled"
      class="relative w-20 h-20 rounded-full flex items-center justify-center transition-all duration-200 focus:outline-none focus:ring-4 focus:ring-opacity-50"
      :class="buttonClass"
    >
      <Icon
        v-if="!isRecording"
        name="heroicons:play"
        class="w-8 h-8"
      />
      <div
        v-else
        class="w-8 h-8 bg-white rounded-sm"
      />
    </button>
    
    <div class="text-center">
      <p class="text-sm font-medium text-gray-900 dark:text-white">
        {{ isRecording ? 'Recording...' : 'Tap to Record' }}
      </p>
      <p v-if="duration > 0" class="text-xs text-gray-500 dark:text-gray-400">
        {{ formatDuration(duration) }}
      </p>
    </div>
    
    <div v-if="isRecording" class="flex items-center space-x-2">
      <div class="w-2 h-2 bg-red-500 rounded-full animate-pulse" />
      <span class="text-sm text-red-600 dark:text-red-400">Recording</span>
    </div>
    
    <div v-if="waveformData.length > 0" class="w-full max-w-md">
      <div class="flex items-end space-x-1 h-8">
        <div
          v-for="(bar, index) in waveformData"
          :key="index"
          class="bg-blue-500 rounded-sm"
          :style="{ height: `${Math.max(bar * 100, 5)}%` }"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps<{
  disabled?: boolean;
  maxDuration?: number; // in seconds
}>();

const emit = defineEmits<{
  start: [];
  stop: [audioBlob: Blob];
  error: [error: string];
}>();

const isRecording = ref(false);
const duration = ref(0);
const waveformData = ref<number[]>([]);

let mediaRecorder: MediaRecorder | null = null;
let audioChunks: Blob[] = [];
let durationInterval: number | null = null;
let waveformInterval: number | null = null;
let audioContext: AudioContext | null = null;
let analyser: AnalyserNode | null = null;
let dataArray: Uint8Array | null = null;

const buttonClass = computed(() => {
  if (props.disabled) {
    return 'bg-gray-300 text-gray-500 cursor-not-allowed';
  }
  if (isRecording.value) {
    return 'bg-red-500 hover:bg-red-600 text-white focus:ring-red-300';
  }
  return 'bg-blue-500 hover:bg-blue-600 text-white focus:ring-blue-300';
});

const toggleRecording = async () => {
  if (isRecording.value) {
    stopRecording();
  } else {
    await startRecording();
  }
};

const startRecording = async () => {
  try {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    
    mediaRecorder = new MediaRecorder(stream);
    audioChunks = [];
    
    mediaRecorder.ondataavailable = (event) => {
      if (event.data.size > 0) {
        audioChunks.push(event.data);
      }
    };
    
    mediaRecorder.onstop = () => {
      const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
      emit('stop', audioBlob);
      
      // Stop all tracks
      stream.getTracks().forEach(track => track.stop());
    };
    
    mediaRecorder.start();
    isRecording.value = true;
    duration.value = 0;
    
    // Start duration timer
    durationInterval = window.setInterval(() => {
      duration.value++;
      if (props.maxDuration && duration.value >= props.maxDuration) {
        stopRecording();
      }
    }, 1000);
    
    // Setup audio visualization
    setupAudioVisualization(stream);
    
    emit('start');
  } catch (error) {
    console.error('Error starting recording:', error);
    emit('error', 'Failed to access microphone');
  }
};

const stopRecording = () => {
  if (mediaRecorder && isRecording.value) {
    mediaRecorder.stop();
    isRecording.value = false;
    
    if (durationInterval) {
      clearInterval(durationInterval);
      durationInterval = null;
    }
    
    if (waveformInterval) {
      clearInterval(waveformInterval);
      waveformInterval = null;
    }
    
    if (audioContext) {
      audioContext.close();
      audioContext = null;
    }
  }
};

const setupAudioVisualization = (stream: MediaStream) => {
  try {
    audioContext = new AudioContext();
    const source = audioContext.createMediaStreamSource(stream);
    analyser = audioContext.createAnalyser();
    
    analyser.fftSize = 256;
    const bufferLength = analyser.frequencyBinCount;
    dataArray = new Uint8Array(bufferLength);
    
    source.connect(analyser);
    
    waveformInterval = window.setInterval(() => {
      if (analyser && dataArray) {
        analyser.getByteFrequencyData(dataArray);
        
        // Downsample for visualization
        const step = Math.floor(dataArray.length / 20);
        const downsampled = [];
        for (let i = 0; i < dataArray.length; i += step) {
          downsampled.push(dataArray[i] / 255);
        }
        waveformData.value = downsampled;
      }
    }, 100);
  } catch (error) {
    console.error('Error setting up audio visualization:', error);
  }
};

const formatDuration = (seconds: number): string => {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

onUnmounted(() => {
  if (durationInterval) {
    clearInterval(durationInterval);
  }
  if (waveformInterval) {
    clearInterval(waveformInterval);
  }
  if (audioContext) {
    audioContext.close();
  }
});
</script>
