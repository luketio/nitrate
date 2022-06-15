<script lang="ts">
	import { onMount } from "svelte";
	import { readDir, createDir, BaseDirectory } from "@tauri-apps/api/fs";
	import Hide from "./components/Hide.svelte";
	import Refresh from "./components/Refresh.svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import Board from "./components/Board.svelte";

	let emojis: string[];

	onMount(async () => {
		await verifyDir();

		await invoke("resize_all");

		emojis = await invoke("get_filenames", { resized: true, absolute: true });
	})

	const verifyDir = async () => {
		try {
			await readDir("Nitrate/emojis", { dir: BaseDirectory.Data});
		} catch (err) {
			await createDir("Nitrate/emojis", { dir: BaseDirectory.Data, recursive: true});
		}
  	}
</script>

<header>
	<Hide />
	<Refresh bind:emojis />
</header>

<main>
	<Board emojis={emojis} />
</main>

<style lang="scss">
	header {
		margin: 6px;
	}

	main {
		margin-left: 6px;
		margin-right: 6px;
	}
</style>
