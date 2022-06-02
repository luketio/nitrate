<script lang="ts">
	import hotkeys from "hotkeys-js";
	import { appWindow } from "@tauri-apps/api/window";

	import Emoji from "./components/Emoji.svelte";
	import Hide from "./components/Hide.svelte";

	hotkeys("alt+r", function(event, handler) {
		event.preventDefault();
		appWindow.minimize();
	});
</script>

<header>
	<Hide />
</header>

<main>
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
	<Emoji img="svelte.png" />
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
