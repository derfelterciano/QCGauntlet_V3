/** @type {import('tailwindcss').Config} */
export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			fontFamily: {
				sans: ["Monaspace", "Arial"],
				mono: ["Fira Code", "ui-monospace", "SFMono-Regular"],
			},
		},
	},
	darkMode: "class",
	plugins: [],
};
