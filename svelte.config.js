import sveltePreprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: sveltePreprocess(),
  kit: {
    adapter: adapter({
      pages: 'dist',
    }),
    prerender: {
      default: false,
    },
    files: {
      template: 'index.html'
    },
    package: {
      dir: 'public',
    }
  },
};

export default config;