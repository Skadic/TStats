<script lang="ts">
	import type { PoolBracket } from '$lib/api/pool';
	import PoolMapCard from './PoolMapCard.svelte';

	export let tournamentId: number;
	export let stage: number;
	export let brackets: PoolBracket[];
</script>

<div>
	{#each brackets as bracket, bracketIndex}
		<div class="p-2">
			<div class="flex flex-col gap-2 px-4 bg-bg-500 shadow-xl rounded-xl py-4 pb-6">
				{#if bracket.maps !== undefined}
					<h2 class="text-3xl font-bold">{bracket.name}</h2>
					{#each bracket.maps.maps as map, i}
						<div class=" transition-all duration-200 hover:scale-105">
							<PoolMapCard
								linkData={{ bracket: bracketIndex, stage, tournamentId }}
								bracketName={brackets[bracketIndex].name}
								mapOrder={i}
								{map}
							/>
						</div>
					{/each}
				{:else}
					Maps undefined
				{/if}
			</div>
		</div>
	{/each}
</div>
