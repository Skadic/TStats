<script lang="ts">
	import type { PoolBracket } from '$lib/Pool';
	import type { Stage } from '$lib/Stage';
	import PoolList from './PoolList.svelte';
	export let stage: Stage;
	export let poolBrackets: PoolBracket[];
	export let hasBestOf: boolean;
</script>

<div class="rounded-lg pb-4 bg-bg-400 duration-200">
	<div class="flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all">
		<h1 class="text-3xl font-bold">{stage.name}</h1>
		{#if hasBestOf} <h2 class="flex-1 text-right">Best of {stage.bestOf}</h2> {/if}
	</div>
	<div class="px-4 py-2">
		<PoolList brackets={poolBrackets} />
	</div>
</div>
