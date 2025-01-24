<script lang="ts" context="module">
	import { open } from "@tauri-apps/plugin-dialog";
	// Define UserConfig type
	import FilePrompt from "./FilePrompt.svelte";

	export interface UserConfig {
		primary_ds: string;
		secondary_ds: string;
		threshold: number;
		controls: ControlDefinitions | null;
		compound_name_col: string;
		well_location_col: string;
		plate_name_col: string;
	}

	export interface ControlDefinitions {
		control_column: string;
		control_blocks: ControlBlock[];
	}

	export interface ControlBlock {
		name: string;
		control_wells: string[];
	}

	export let userConfig: UserConfig = {
		primary_ds: "",
		secondary_ds: "",
		threshold: 0.5,
		controls: null,
		compound_name_col: "",
		well_location_col: "",
		plate_name_col: "",
	};

	let assayLayout: boolean[] = Array(384).fill(false); // 384 well default make 96 wells an option later

	// Handle file input

	function handleFiles(
		key: "primary_ds" | "secondary_ds",
		file: string,
	): void {
		userConfig[key] = file;
	}

	// Update threshold
	function updateThresh(value: string) {
		userConfig.threshold = parseFloat(value);
	}

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault(); // Explicitly prevent default behavior
		console.log("Form submitted:", userConfig);
	}
</script>

<form class="inputForm" on:submit={handleSubmit}>
	<h3 class="flex justify-center">User Configuration</h3>

	<!-- File selection -->
	<div class="file-selection">
		<label class="mb-3">
			Primary Dataset:
			<FilePrompt
				onFileSelect={(file) => handleFiles("primary_ds", file)}
			/>
		</label>
		<label>
			Secondary Dataset:
			<FilePrompt
				onFileSelect={(file) => handleFiles("secondary_ds", file)}
			/>
		</label>
	</div>

	<div class="threshold"></div>

	<button
		class="flex justify-center w-1/2 my-5 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 border border-green-700 rounded"
		type="submit">Save Configuration</button
	>
</form>

<style>
	.inputForm {
		@apply flex flex-col items-center rounded-lg border-white border-2 p-[2px] ml-2 bg-sky-900;
		flex-shrink: 0;
		min-width: 500px;
		width: 35%;
	}
	.file-selection {
		display: block;
		width: 100%;
	}

	label {
		display: block;
	}
</style>
