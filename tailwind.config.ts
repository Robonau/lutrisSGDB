const colors = require('tailwindcss/colors');
const svelte_ux = require('svelte-ux/plugins/tailwind.cjs');
import { screens } from './src/lib';

/** @type {import('tailwindcss').Config}*/
const config = {
	content: ['./src/**/*.{html,svelte}', './node_modules/svelte-ux/**/*.{svelte,js}'],
	theme: {
		extend: {
			aspectRatio: {
				cover: '210 / 297'
			},
			width: {
				'1/7': 'calc(100% / 7)',
				'1/8': '12.5%',
				'1/12': 'calc(100% / 12)',
				'1/16': 'calc(100% / 16)',
				'1/18': 'calc(100% / 18)'
			},
			maxWidth: {
				'4/5': '80%',
				'1/3': 'calc(100% / 3)'
			},
			borderRadius: {
				olg: '0.47rem'
			},
			screens
		}
	},
	variants: {
		extend: {}
	},
	plugins: [svelte_ux]
};

module.exports = config;
