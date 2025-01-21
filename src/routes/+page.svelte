<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import InputDiv, {
		type UserConfig,
	} from "../components/userInputs/InputDiv.svelte";

	let name = $state("");
	let greetMsg = $state("");

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke("greet", { name });
	}
</script>

<main class="container">
	<h1>QC Gauntlet</h1>
	<InputDiv></InputDiv>
</main>

<style>
	.container {
		@apply font-sans text-white;
	}
</style>
