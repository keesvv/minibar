import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import icons from "unplugin-icons/vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), icons({ compiler: "svelte" })],
});
