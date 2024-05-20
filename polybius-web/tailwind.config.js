module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "index.html"],
  },
  darkMode: "media", // 'media' or 'selector'
  theme: {
    extend: {
      colors: {
        primary: {
          '50': '#f2fbf5',
          '100': '#e0f8e7',
          '200': '#c2f0d1',
          '300': '#93e2ae',
          '400': '#50c878',
          '500': '#36b15f',
          '600': '#27924c',
          '700': '#22733d',
          '800': '#205b34',
          '900': '#1c4b2e',
          '950': '#0a2916',
          '1000': '#0e110f'
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
};