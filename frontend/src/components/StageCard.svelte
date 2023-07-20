<script lang="ts">
	import type { Stage } from '$lib/Stage';
	export let stage: Stage;
	export let has_best_of: boolean;
</script>

<div class="stageCard">
	<a href="/stage/{stage.id.split(':')[1]}">
		<h1>{stage.name}</h1>
		{#if has_best_of} <h2>Best of {stage.best_of}</h2> {/if}
	</a>
</div>

<style>
	h1 {
		@apply text-3xl font-bold;
	}

	h2 {
		@apply flex-1 text-right;
	}

	.stageCard {
		@apply text-ctp-base rounded-lg p-2 bg-gradient-to-br from-ctp-pink to-ctp-mauve;
	}

	a {
		@apply flex;
	}
</style>
