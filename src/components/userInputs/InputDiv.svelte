<script lang="ts" context="module">
  import { preventDefault } from "svelte/legacy";

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
	<h3>User Configuration</h3>

	<!-- File selection -->
	<div class="file-selection">
		<label>
			Primary Dataset:
			<input
				type="file"
				on:change={(e) => handleFiles(e, "primary_ds")}
			/>
		</label>
	</div>

	<button type="submit">Save Configuration</button>
</form>

<style>
	.inputForm {
		padding: 1rem;
		border: 1px solid #ccc;
		border-radius: 8px;
		background-color: #f9f9f9;
	}
</style>
