import typography from '@tailwindcss/typography'
import forms from '@tailwindcss/forms'
import aspectRatio from '@tailwindcss/aspect-ratio'
import containerQueries from '@tailwindcss/container-queries'

/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'selector',
  // TODO: maybe we can reuse mui-selector, need research
  // darkMode: ['selector', '[data-mui-color-scheme="dark"]'],
  corePlugins: {
    preflight: false,
  },
  content: ['./app/**/*.{js,ts,jsx,tsx,mdx}', './components/**/*.{js,ts,jsx,tsx}'],
  theme: {
    fontFamily: {
      display: ['Roboto Mono', 'Menlo', 'monospace'],
      body: ['Roboto Mono', 'Menlo', 'monospace'],
      inter: ['Inter', 'sans-serif'],
      caveat: ['Caveat', 'cursive'],
      sans: ['var(--font-inter)', 'ui-sans-serif', 'system-ui', 'sans-serif'],
    },
    extend: {
      typography: () => ({
        dark: {
          css: {
            color: 'white',
          },
        },
      }),
    },
  },
  plugins: [typography, forms, aspectRatio, containerQueries],
}
