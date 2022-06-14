<script lang="ts">
	import Emoji from "./Emoji.svelte";

	export let emojis: string[] = [];
</script>

{#if emojis}
	{#each emojis as emoji}
		<Emoji img={emoji} />
	{/each}
{/if}
