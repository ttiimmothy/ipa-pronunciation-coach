import { createApp } from 'vue';
import { createPinia } from 'pinia';
import VueQueryPlugin, { queryClient } from './plugins/query';
import App from './App.vue';

const app = createApp(App);

// Setup Pinia
const pinia = createPinia();
app.use(pinia);

// Setup TanStack Query
app.use(VueQueryPlugin, { queryClient });

app.mount('#app');
