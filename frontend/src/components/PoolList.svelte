<script lang="ts">
	import type { PoolBracket } from '$lib/api/pool';
	import PoolMapCard from './PoolMapCard.svelte';

	export let brackets: PoolBracket[];
	let maps = brackets[0].maps?.maps!;
</script>

<div class="">
	{#each brackets as bracket, bracketIndex}
		<div class="py-2">
			<h1 class="text-3xl font-bold py-1">{bracket.name}</h1>
			<div class="flex flex-col gap-2 px-4">
				{#if bracket.maps !== undefined}
					{#each bracket.maps.maps as map, i}
						<PoolMapCard bracketOrder={i + 1} bracketName={brackets[bracketIndex].name} {map} />
					{/each}
				{:else}
					Maps undefined
				{/if}
			</div>
		</div>
	{/each}
</div>
