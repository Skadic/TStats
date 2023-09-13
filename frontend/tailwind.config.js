const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				bg: {
					DEFAULT: '#1a1d1c',
					100: '#090a09',
					200: '#121312',
					300: '#1a1d1c',
					400: '#232625',
					500: '#2c302e',
					600: '#545b58',
					700: '#7d8782',
					800: '#a8afab',
					900: '#d4d7d5'
				},
				text: {
					DEFAULT: '#dcedff',
					100: '#002e5f',
					200: '#005cbe',
					300: '#1e8bff',
					400: '#7cbcff',
					500: '#dcedff',
					600: '#e2f0ff',
					700: '#eaf4ff',
					800: '#f1f8ff',
					900: '#f8fbff'
				},
				secondary: {
					DEFAULT: '#94b0da',
					100: '#132137',
					200: '#25426d',
					300: '#3863a4',
					400: '#5e88c8',
					500: '#94b0da',
					600: '#aac0e2',
					700: '#bfd0e9',
					800: '#d4e0f0',
					900: '#eaeff8'
				},
				accent: {
					DEFAULT: '#f6ae2d',
					100: '#382502',
					200: '#704a05',
					300: '#a76f07',
					400: '#df9409',
					500: '#f6ae2d',
					600: '#f8bf57',
					700: '#facf81',
					800: '#fbdfab',
					900: '#fdefd5'
				}
			},
			minWidth: {
				'1/2': '50%',
				'1/3': '33%',
			}
		}
	},
	plugins: [
		require('@catppuccin/tailwindcss')({
			prefix: 'ctp',
			defaultFlavour: 'mocha'
		})
	]
};
