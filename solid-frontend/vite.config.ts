import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 5173,
    host: true,
    allowedHosts: ["localdev.skadic.moe"]
  },
  build: {
    target: 'esnext',
  },
});
