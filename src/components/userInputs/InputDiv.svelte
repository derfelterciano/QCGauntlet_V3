<script lang="ts">
	// Define UserConfig type
	import FilePrompt from "./FilePrompt.svelte";
	import AddMeta from "./AddMeta.svelte";
	import type {
		UserConfig,
		ControlDefinitions,
		ControlBlock,
	} from "../../types/types";

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

	let metaInfoList: string[] = [];
	function handleAddMetaInfo(): void {
		metaInfoList = [...metaInfoList, ""];
	}

	function handleUpdateMetaInfo(index: number, value: string): void {
		metaInfoList = metaInfoList.map((item, i) =>
			i === index ? value : item,
		);
	}

	function handleRemoveMetaInfo(index: number): void {
		metaInfoList = metaInfoList.filter((_, i) => i !== index);
	}

	// Update threshold
	function updateThresh(value: string) {
		userConfig.threshold = parseFloat(value);
	}

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault(); // Explicitly prevent default behavior
		console.log("Form submitted:", userConfig);
		console.log("yuh", metaInfoList);
	}
</script>

<form class="inputForm" on:submit={handleSubmit}>
	<h3 class="flex justify-center font-bold text-lg">User Configuration</h3>

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

	<!-- Meta information -->
	<div class="meta-info">
		<h5 class="flex justify-center font-bold text-sm">Meta Information</h5>

		<!-- Essential Meta Information-->
		<label class="block flex flex-col items-start my-2">
			Compound Name Column:
			<input
				class="border-2 rounded text-black w-[250px]"
				type="text"
				bind:value={userConfig.compound_name_col}
				placeholder="Enter Compound Column"
			/>
		</label>
		<label class="block flex flex-col items-start my-2">
			Well Location Column:
			<input
				class="border-2 rounded text-black w-[250px]"
				type="text"
				bind:value={userConfig.well_location_col}
				placeholder="Enter Well Column"
			/>
		</label>
		<label class="block flex flex-col items-start my-2">
			Plate Name Column:
			<input
				class="border-2 rounded text-black w-[250px]"
				type="text"
				bind:value={userConfig.plate_name_col}
				placeholder="Enter Plate Name Column"
			/>
		</label>

		<!--Additional Meta Information-->
		<AddMeta
			addMetaInfoList={metaInfoList}
			onAddMetaInfo={handleAddMetaInfo}
			onRemoveMetaInfo={handleRemoveMetaInfo}
			onUpdateMetaInfo={handleUpdateMetaInfo}
		/>
	</div>

	<div class="threshold"></div>

	<button
		class="flex justify-center w-1/2 my-5 bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 border border-green-700 rounded"
		type="submit">Save Configuration</button
	>
</form>

<style>
	.inputForm {
		@apply flex flex-col items-center rounded-lg border-white border-2 p-[10px] ml-2 bg-sky-900;
		flex-shrink: 0;
		min-width: 500px;
		width: 35%;
	}
	.file-selection {
		display: block;
		width: 100%;
	}

	.meta-info {
		@apply flex flex-col items-center border-2 border-white rounded-md px-2 py-1 my-5;
		width: 100%;
	}

	/* label { */
	/* 	@apply flex flex-col items-start w-full; */
	/* 	display: block; */
	/* } */
</style>
