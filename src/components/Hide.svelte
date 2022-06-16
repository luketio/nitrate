<script lang="ts">
	import { appWindow } from "@tauri-apps/api/window";
	const hide = () => {
		/**
		 * Minimizes the window when clicked
		*/
		appWindow.minimize();
	};
</script>

<button on:click={hide}>
	HIDE
</button>

<style lang="scss">
	button {
		font-size: 13px;
		padding: 4px;
		border-radius: 1vw;
		border: none;
		width: 50px;
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
