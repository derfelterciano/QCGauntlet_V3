<script lang="ts">
	import { open } from "@tauri-apps/plugin-dialog";

	export let label: string = "Select File";
	export let accept: string[] = [];
	export let onFileSelect: (file: string) => void;

	function truncatePath(path: string, max_len: number): string {
		if (path.length <= max_len) return path;

		const parts = path.split("/");
		const fileName = parts.pop();
		const dirPath = parts.join("/");

		const visiblePathLen = Math.max(0, max_len - fileName?.length - 5);
		const truncDir = dirPath.slice(0, visiblePathLen) + "...";

		return `${truncDir}/${fileName}`;
	}

	let disp_text: string = "No file Selected";
	async function handleClick(): Promise<void> {
		const file = await open({
			multiple: false,
			directory: false,
		});

		if (file && typeof file === "string") {
			onFileSelect(file);
			disp_text = file;
		}
	}
</script>

<div class="file-select">
	<button
		id="file-dialog"
		class="rounded-md bg-gray-800 hover:bg-gray-500"
		on:click={handleClick}>{label}</button
	>
	<p id="file-text">{truncatePath(disp_text, 40)}</p>
</div>

<style>
	.file-select {
		@apply flex items-center border rounded-md  bg-gray-700;
		width: 75%;
		height: 40px;
		flex-shrink: 0;
	}
	#file-dialog {
		@apply flex justify-center items-center px-4 mr-2 text-center;
		height: 100%;
		flex-shrink: 0;
	}
	#file-text {
		flex-shrink: 0;
	}
</style>
