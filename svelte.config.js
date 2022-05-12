import sveltePreprocess from "svelte-preprocess";
import adapter from "@sveltejs/adapter-static";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    sveltePreprocess({
      postcss: true,
    }),
  ],
  kit: {
    vite: {
      build: { target: ['esnext'], },
    },
    adapter: adapter({
      pages: "dist",
    }),
    prerender: {
      default: false,
    },
    files: {
      template: "index.html",
    },
  },
};

export default config;
