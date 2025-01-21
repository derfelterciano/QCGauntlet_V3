<script lang="ts" context="module">
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
	function handleFiles(event: Event, key: keyof UserConfig): void {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];

		if (file) {
			const reader = new FileReader();
			reader.onload = () => {
				if (key === "primary_ds" || key === "secondary_ds") {
					userConfig[key] = reader.result as string; // Assign content
				}
			};
			reader.readAsText(file); // You can also use readAsArrayBuffer, etc.
		}
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
		<label>
			Primary Dataset:
			<input
				type="file"
				on:change={(e) => handleFiles(e, "primary_ds")}
			/>
		</label>
		<label>
			Secondary Dataset:
			<input
				type="file"
				on:change={(e) => handleFiles(e, "secondary_ds")}
			/>
		</label>
	</div>

	<button
		class="flex justify-center m-[5px] bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 border border-green-700 rounded"
		type="submit">Save Configuration</button
	>
</form>

<style>
	.inputForm {
		@apply rounded-lg border-black border-2 p-[2px] ml-2;
	}
</style>
