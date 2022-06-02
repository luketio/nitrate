<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri"
	import { appWindow } from "@tauri-apps/api/window";
	import { ASSETS_PATH } from "../lib/constants";

	export let img: string;
	let filename = `${ASSETS_PATH}/${img}`;

	const copy = () => {
		invoke("copy_image", {
			filename: filename,
		});
		appWindow.minimize();
	};
</script>

<button on:click={copy}>
	<img src={`emojis/${img}`} alt={`emoji: ${img}`}/>
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
