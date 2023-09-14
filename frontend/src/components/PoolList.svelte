<script lang="ts">
	import type { PoolBracket } from '$lib/Pool';
	import PoolMapCard from './PoolMapCard.svelte';

	export let brackets: PoolBracket[];
</script>

<div>
	{#each brackets as bracket}
		<div class="py-2">
			<h1 class="text-3xl font-bold py-1">{bracket.name}</h1>
			<div class="flex flex-col gap-2 px-4">
				{#each bracket.maps as map}
					<PoolMapCard {map} />
				{/each}
			</div>
		</div>
	{/each}
</div>
