<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	export let emojis = [];

	const refresh = async () => {
		/**
		 * Resizes, and refreshes UI every time the button is clicked
		*/
		await invoke("resize_all");
		emojis = await invoke("get_filenames", { resized: true, absolute: true });
	}
</script>

<button on:click={refresh}>
	REFRESH
</button>

<style lang="scss">
	button {
		font-size: 13px;
		padding: 4px;
		border-radius: 1vw;
		border: none;
		width: 70px;
		cursor: pointer;
		transition: background-color 0.2s;

		&:hover {
			background-color: $text-color-hover;
		}
		&:active {
			color: white;
		}
	}
</style>
