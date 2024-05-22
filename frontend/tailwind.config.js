/** @type {import('tailwindcss').Config} */
export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				sans: ['Fira Sans', 'sans-serif'],
				mono: ['Fira Mono', 'monospace']
			}
		}
	},
	plugins: []
};
