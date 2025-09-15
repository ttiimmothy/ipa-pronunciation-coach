// @ts-check
import { defineConfig } from 'astro/config';
import vue from '@astrojs/vue';
import tailwind from '@astrojs/tailwind';
import Icons from 'unplugin-icons/vite';

// https://astro.build/config
export default defineConfig({
  integrations: [vue(), tailwind()],
  output: 'static',
  server: {
    port: 4320
  },
  vite: {
    define: {
      'process.env': {}
    },
    plugins: [
      Icons({
        compiler: 'vue3',
        autoInstall: true,
      })
    ]
  }
});
