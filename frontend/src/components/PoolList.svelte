<script lang="ts">
	import { Difficulty, PoolBracket, PoolMap } from '$lib/Pool';
	import PoolMapCard from './PoolMapCard.svelte';

	export let brackets: PoolBracket[];

	brackets = [
		new PoolBracket('NM', [
			new PoolMap(
				'Demetori',
				'Kagayaku Hari no Kobito-zoku ~ Counter-Attack of the Weeeeeeeeeeeeeeeeeeak',
				347021,
				765873,
				new Difficulty(6.51, 366, 184, 4.5, 9.5, 9, 6.5)
			),
			new PoolMap(
				'Demetori',
				'Kagayaku Hari no Kobito-zoku ~ Counter-Attack of the Weak',
				347021,
				765873,
				new Difficulty(6.51, 366, 184, 4.5, 9.5, 9, 6.5)
			)
		]),
		new PoolBracket('HD', [
			new PoolMap(
				'Memme',
				'Avalanche',
				611095,
				1290095,
				new Difficulty(6.12, 118, 175, 4.5, 9.5, 9, 6)
			)
		])
	];
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
