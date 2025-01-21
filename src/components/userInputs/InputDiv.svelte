<script lang="ts" context="module">
	import { open } from "@tauri-apps/plugin-dialog";

	// Define UserConfig type
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
	async function handleFiles(
		key: "primary_ds" | "secondary_ds",
	): Promise<void> {
		const file = await open({
			multiple: false,
			directory: false,
		});

		if (file && typeof file === "string") {
			userConfig[key] = file;
		}
	}
	// function handleFiles(event: Event, key: keyof UserConfig): void {
	// 	const target = event.target as HTMLInputElement;
	// 	const file = target.files?.[0];
	//
	// 	if (file && (key === "primary_ds" || key === "secondary_ds")) {
	// 		userConfig[key] = file.name;
	// 	}
	// }

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
		<label class="block mb-2 text-md font-medium text-white">
			Primary Dataset:
			<input
				class="block w-full mb-5 text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer bg-gray-50 text-gray-400 focus:outline-none bg-gray-700 border-gray-600 placeholder-gray-400"
				type="file"
				on:change={(e) => handleFiles("primary_ds")}
				accept=".csv,.tsv,.txt"
			/>
		</label>

		<label class="block mb-2 text-md font-medium text-white">
			Secondary Dataset:
			<input
				class="block w-full mb-5 text-sm text-gray-900 border border-gray-300 rounded-lg cursor-pointer bg-gray-50 text-gray-400 focus:outline-none bg-gray-700 border-gray-600 placeholder-gray-400"
				type="file"
				on:change={(e) => handleFiles("secondary_ds")}
				accept=".csv,.tsv,.txt"
			/>
		</label>
	</div>

	<div class="threshold"></div>

	<button
		class="flex justify-center m-[5px] bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 border border-green-700 rounded"
		type="submit">Save Configuration</button
	>
</form>

<style>
	.inputForm {
		@apply rounded-lg border-white border-2 p-[2px] ml-2 bg-sky-900;
	}

	label {
		display: block;
	}
</style>
