<script lang="ts">
	import type { PoolBracket } from '$lib/Pool';
	import type { Stage } from '$lib/Stage';
	import type { ExtendedTournament } from '$lib/Tournament';
	import StageCard from '../../../components/StageCard.svelte';
	import TournamentInfo from '../../../components/TournamentInfo.svelte';
	import type { TournamentResult } from './+page';

	export let data: TournamentResult;
	let extTournament: ExtendedTournament = data.tournament;
	let stages: Stage[] = extTournament.stages;
	let brackets: PoolBracket[][] = data.poolBrackets;
</script>

<div class="bg-bg rounded-xl flex flex-col gap-8 p-5">
	<div
		class=" flex flex-col justify-center items-center shadow-bg-100 shadow-md rounded-2xl lg:rounded-lg p-3"
	>
		<h1 class="text-5xl font-bold text-center p-3 pb-5">{extTournament.tournament.name}</h1>
		<TournamentInfo data={extTournament} />
	</div>
	<div>
		<h2 class="text-4xl font-bold p-3 pb-5">Stages</h2>
		{#each stages as stage, i}
			<div class="flex-1 my-3">
				<StageCard
					stage={{
						name: stage.name,
						tournamentId: extTournament.tournament.id,
						bestOf: stage.bestOf,
						stageOrder: i
					}}
					poolBrackets={brackets[i]}
					hasBestOf={extTournament.tournament.formatTournamentFormat().includes('v')}
				/>
			</div>
		{:else}
			<div class="p-2 min-w-full">No stages found</div>
		{/each}
	</div>
</div>

<style>
</style>
