<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/tauri";
	import { readDir, createDir, BaseDirectory } from "@tauri-apps/api/fs";
	import Hide from "./components/Hide.svelte";
	import Refresh from "./components/Refresh.svelte";
	import Board from "./components/Board.svelte";

	let emojis: string[];

	onMount(async () => {
		/**
		 * Runs once on the first app load
		 * @returns {Promise<void>}
		*/
		await verifyDir();

		/**
		 * Resizes all images that exist in the emojis/ directory but not resized/
		*/
		await invoke("resize_all");

		/**
		 * Gets all emoji filenames
		 * Later bound to the Refresh component so that it can refresh the emojis array
		*/
		emojis = await invoke("get_filenames", { resized: true, absolute: true });
	})

	const verifyDir = async () => {
		/**
		 * Verifies that required nitrate directories exist, and if they do not, creates them.
		 * @returns {Promise<void>}
		*/
		try {
			await readDir("Nitrate/emojis", { dir: BaseDirectory.Data});
		} catch (err) {
			await createDir("Nitrate/emojis", { dir: BaseDirectory.Data, recursive: true});
		}

		try {
			await readDir("Nitrate/resized", { dir: BaseDirectory.Data});
		} catch (err) {
			await createDir("Nitrate/resized", { dir: BaseDirectory.Data, recursive: true});
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
