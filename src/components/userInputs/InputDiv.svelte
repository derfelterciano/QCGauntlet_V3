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
	function handleFiles(event: Event, key: keyof UserConfig) {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0].text();
		userConfig[key] = file;
	}

	// Update threshold
	function updateThresh(value: string) {
		userConfig.threshold = parseFloat(value);
	}
</script>

<div class="inputDiv">
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
</div>

<style>
	.inputDiv {
		padding: 1rem;
		border: 1px solid #ccc;
		border-radius: 8px;
		background-color: #f9f9f9;
	}
</style>
