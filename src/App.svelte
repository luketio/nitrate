<script lang="ts">
	import hotkeys from 'hotkeys-js';
	import { appWindow } from "@tauri-apps/api/window";

	import logo from "./assets/svelte.png";
	import Emoji from "./components/Emoji.svelte";
	import Hide from "./components/Hide.svelte";

	hotkeys('alt+r', function(event, handler){
		event.preventDefault();
		appWindow.minimize();
	});
</script>

<main>
	<img src={logo} alt="Svelte Logo" />

	<Emoji img="svelte.png" />
	<Hide />
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}

	img {
		height: 16rem;
		width: 16rem;
	}
</style>
