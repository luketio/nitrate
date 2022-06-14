<script lang="ts">
	import { onMount } from "svelte";
	import { writeText, readText } from '@tauri-apps/api/clipboard';
	import { readBinaryFile, BaseDirectory } from "@tauri-apps/api/fs";
	import { invoke } from "@tauri-apps/api/tauri"
	import { appWindow } from "@tauri-apps/api/window";
	import { ASSETS_PATH } from "../lib/constants";

	export let img: string;

	// del
	const filename = `${ASSETS_PATH}/resized/${img}`;
	let img_data;

	const copy = () => {
		invoke("copy_image", {
			filename: filename,
		});
		appWindow.minimize();
	};

	onMount(async () => {
		const bytes = await readBinaryFile(`Nitrate/resized/${img}`, { dir: BaseDirectory.Data });
		img_data = "data:image/png;base64," + btoa(String.fromCharCode(...new Uint8Array(bytes)));
	});
</script>

<button on:click={copy}>
	<img src={img_data} alt="img"/>
</button>

<style lang="scss">
	button {
		cursor: pointer;

		border: none;
		padding: 4px;
		border-radius: 1.5vw;
		border: none;

		transition: background-color 0.2s;

		&:hover {
			background-color: $text-color-hover;
			img {
				background-color: $text-color-hover;
			}
		}
	}

	img {
		transition: background-color 0.2s;

		height: 50px;
		width: 50px;
	}
</style>
