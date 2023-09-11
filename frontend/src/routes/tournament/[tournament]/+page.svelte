<script lang="ts">
	import type { Stage } from '$lib/Stage';
	import { formatRankRangeDetailed, formatTournamentFormat } from '$lib/Tournament';
	import Flag from '../../../components/Flag.svelte';
	import StageCard from '../../../components/StageCard.svelte';
	import TournamentInfo from '../../../components/TournamentInfo.svelte';
	import type { StageResult, TournamentResult } from './+page';

	export let data: TournamentResult;
	let tournament: TournamentResult = data;
	let stages: StageResult[] = data.stages;
	let rankRanges = formatRankRangeDetailed(tournament);
</script>

<h1 class="text-5xl font-bold text-center p-3 mb-5">{tournament.name}</h1>
<div class="bg-bg rounded-xl flex flex-col p-5">
	<div class="shadow-bg-100 shadow-md m-5 mt-0 p-3 flex justify-center">
		<TournamentInfo {data} />
	</div>
	<div />
	<div class="min-w-full">
		{#each stages as stage, i}
			<div class="flex-1 my-3">
				<StageCard
					stage={{
						name: stage.name,
						tournament_id: tournament.id,
						best_of: stage.best_of,
						stage_order: i
					}}
					has_best_of={formatTournamentFormat(tournament).includes('v')}
				/>
			</div>
		{:else}
			<div class="m-2 min-w-full">No stages found</div>
		{/each}
	</div>
</div>

<style>
</style>
