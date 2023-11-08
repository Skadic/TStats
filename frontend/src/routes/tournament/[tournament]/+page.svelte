<script lang="ts">
	import type { PoolBracket } from '$lib/api/pool';
	import type { GetAllStagesResponse } from '$lib/api/stages';
	import type {
		CountryList,
		GetTournamentResponse,
		RankRange,
		Tournament
	} from '$lib/api/tournaments';
	import StageCard from '../../../components/StageCard.svelte';
	import TournamentInfo from '../../../components/TournamentInfo.svelte';

	export let data;
	let tournamentData: GetTournamentResponse = data.tournament;
	let tournament: Tournament = tournamentData.tournament!;
	let rankRanges: RankRange[] = tournamentData.rankRestrictions?.ranges!;
	let countryRestrictions: CountryList = tournamentData.countryRestrictions!;
	let stages: GetAllStagesResponse[] = data.stages;
	let brackets: PoolBracket[] = data.pool ?? [];
</script>

<div class="bg-bg rounded-xl flex flex-col justify-center gap-8 p-5">
	<div
		class=" flex flex-col justify-center items-center shadow-bg-100 shadow-md rounded-2xl lg:rounded-lg p-3"
	>
		<h1 class="text-4xl lg:text-5xl font-bold text-center p-3 pb-5">
			{tournamentData.tournament?.name}
		</h1>
		<TournamentInfo {tournament} {rankRanges} {countryRestrictions} />
	</div>
	<div style="">
		<h2 class="text-4xl font-bold p-3 pb-5">Stages</h2>
		{#each stages as stage, i}
			<div class="flex-1 my-3">
				<StageCard
					stage={{
						name: stage.stage?.name ?? 'unnamed stage',
						bestOf: stage.stage?.bestOf ?? -1,
						stageOrder: i
					}}
					poolBrackets={brackets}
					hasBestOf={(stage.stage?.bestOf ?? -1) > 0}
				/>
			</div>
		{:else}
			<div class="p-2 min-w-full">No stages found</div>
		{/each}
	</div>
</div>

<style>
</style>
