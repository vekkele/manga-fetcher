const config = {
  mode: 'jit',
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {},
  },

  plugins: [require("daisyui")],
  corePlugins: {
    outline: false,
  }
};

module.exports = config;
