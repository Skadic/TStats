/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors')

module.exports = {
	content: ['./src/**/*.{astro,html,js,jsx,md,svelte,ts,tsx,vue}'],
    media: false,
    mode: 'jit',
	theme: {
		extend: {
            fontFamily: {
                'roboto': ['Roboto', 'sans-serif'],
                'nunito': ['Nunito', 'sans-serif'],
            },
            colors: {
                bgdark: '#242424',
                bglight: '#404040',
                primary: '#FF66AA',
                secondary: '#7D8CC4',
                tx: '#f9f5ff',
                bgobj: '#1b1b3a',
            }
        },
	},
	plugins: [],
    
}
