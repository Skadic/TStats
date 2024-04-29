<script lang="ts">
	import { page } from '$app/stores';
	import type { Beatmap } from '$lib/api/osu.js';
	import type { PoolBracket } from '$lib/api/pool';
	import type { Score } from '$lib/api/scores.js';
	import PoolMapCard from '$lib/components/PoolMapCard.svelte';
	import ScoreCard from '$lib/components/ScoreCard.svelte';

	export let data;
	let tournament: number = Number($page.params.tournament);
	let stage: number = Number($page.params.stage);
	let bracket: number = Number($page.params.bracket);
	let map: number = Number($page.params.map);
	let beatmap: Beatmap = data.beatmap;
	let scores: Score[] = data.scores;
	let brackets: PoolBracket[] = data.brackets;
</script>

<div class="flex flex-col items-center">
	<PoolMapCard bracketName={brackets[bracket].name} map={beatmap} mapOrder={map} />
	
	<div class="flex flex-col gap-6 pt-6 px-2">
		{#each scores as score, i}
			<div>
				<ScoreCard {score} rank={i + 1} />
			</div>
		{/each}
	</div>
</div>
