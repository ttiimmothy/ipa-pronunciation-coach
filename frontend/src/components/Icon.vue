<template>
  <component :is="iconComponent" :class="props.class" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import IconSun from '~icons/heroicons/sun';
import IconMoon from '~icons/heroicons/moon';
import IconClock from '~icons/heroicons/clock';
import IconChartBar from '~icons/heroicons/chart-bar';
import IconCheckCircle from '~icons/heroicons/check-circle';
import IconStar from '~icons/heroicons/star';
import IconPlay from '~icons/heroicons/play';
import IconBookOpen from '~icons/heroicons/book-open';
import IconMicrophone from '~icons/heroicons/microphone';
import IconStop from '~icons/heroicons/stop';
import IconPlus from '~icons/heroicons/plus';

const props = defineProps<{
  name: string;
  class?: string;
}>();

const iconComponent = computed(() => {
  // Map icon names to their unplugin-icons components
  const iconMap: Record<string, any> = {
    'heroicons:sun': IconSun,
    'heroicons:moon': IconMoon,
    'heroicons:clock': IconClock,
    'heroicons:chart-bar': IconChartBar,
    'heroicons:check-circle': IconCheckCircle,
    'heroicons:star': IconStar,
    'heroicons:play': IconPlay,
    'heroicons:book-open': IconBookOpen,
    'heroicons:microphone': IconMicrophone,
    'heroicons:stop': IconStop,
    'heroicons:plus': IconPlus,
  };
  
  return iconMap[props.name] || IconClock;
});
</script>
