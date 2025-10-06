import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': {
        target: 'http://api:8080',
        changeOrigin: true,
      },
      // 👇 add this so <img src="/static/avatars/.."> works in dev
      '/static': {
        target: 'http://api:8080',
        changeOrigin: true,
      },
    },
  },
});
