<script lang="ts">
	import type { Stage } from '$lib/Stage';
	export let stage: Stage;
	export let has_best_of: boolean;
</script>

<div class="text-text rounded-lg p-2 bg-bg-400 container">
	<a class="flex" href="/tournament/{stage.tournament_id}/stage/{stage.stage_order}">
		<h1 class="text-3xl font-bold">{stage.name}</h1>
		{#if has_best_of} <h2 class="flex-1 text-right">Best of {stage.best_of}</h2> {/if}
	</a>
</div>

<style>
	.container {
		transition: all;
		transition-duration: 0.2s;
	}

	.container:hover {
		transform: scale(1.03);
	}
</style>
