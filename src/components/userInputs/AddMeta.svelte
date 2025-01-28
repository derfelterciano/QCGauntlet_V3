<script lang="ts">
	export let addMetaInfoList: string[];
	export let onAddMetaInfo: () => void;
	export let onUpdateMetaInfo: (index: number, value: string) => void;
	export let onRemoveMetaInfo: (index: number) => void;

	// function onAddMetaInfo(): void {
	// 	addMetaInfoList = [...addMetaInfoList, ""];
	// }

	// function onUpdateMetaInfo(index: number, value: string): void {
	// 	addMetaInfoList = addMetaInfoList.map((item, i) =>
	// 		i === index ? value : item,
	// 	);
	// }
	//
	// function onRemoveMetaInfo(index: number): void {
	// 	addMetaInfoList = addMetaInfoList.filter((_, i) => i !== index);
	// }
</script>

<div class="add-meta-info">
	<h6 class="font-bold mb-2 text-center">Additional Meta Information</h6>
	<div class="scroll-area">
		{#each addMetaInfoList as metaInfo, index}
			<div class="scroll-area flex items-center space-x-2 mb-2">
				<input
					class="border-2 rounded text-black p-2 flex-grow"
					type="text"
					value={metaInfo}
					placeholder="Enter Meta Column"
					on:input={(e) =>
						onUpdateMetaInfo(
							index,
							(e.target as HTMLInputElement).value,
						)}
				/>
				<button
					class="bg-red-500 hover:bg-red-600 text-white font-bold py-1 px-3 rounded"
					on:click={() => onRemoveMetaInfo(index)}
					type="button">-</button
				>
			</div>
		{/each}
	</div>
	<!--Add Button-->
	<button
		class="w-1/2 mx-auto bg-blue-500 hover:bg-blue-600 text-white font-bold py-1 px-3 rounded"
		on:click={onAddMetaInfo}
		type="button">+ Add Meta info</button
	>
</div>

<style>
	.add-meta-info {
		@apply flex flex-col border-2 rounded-md p-2;
		width: 100%;
		max-height: 300px; /* Scrollable area */
	}

	.scroll-area {
		max-height: 300px;
		overflow-y: auto;
	}
</style>
