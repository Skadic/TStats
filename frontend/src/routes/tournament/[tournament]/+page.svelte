<script lang="ts">
	import type { GetAllStagesResponse } from '$lib/api/stages';
	import type {
		CountryList,
		GetTournamentResponse,
		RankRange,
		Tournament
	} from '$lib/api/tournaments';
	import { createAccordion, melt, type CreateAccordionProps } from '@melt-ui/svelte';
	import { slide } from 'svelte/transition';
	import StageCard from '../../../components/StageCard.svelte';
	import TournamentInfo from '../../../components/TournamentInfo.svelte';

	export let data;
	let tournamentData: GetTournamentResponse = data.tournament;
	let tournament: Tournament = tournamentData.tournament!;
	let rankRanges: RankRange[] = tournamentData.rankRestrictions?.ranges!;
	let countryRestrictions: CountryList = tournamentData.countryRestrictions!;
	let stages: GetAllStagesResponse[] = data.stages;

	const handleChange: CreateAccordionProps<false>['onValueChange'] = ({ curr, next }) => {
		console.log(next);
		return next;
	};

	const {
		elements: { root: accordion_root, content, item, heading, trigger },
		states: { value }
	} = createAccordion({
		onValueChange: handleChange,
		forceVisible: true
	});
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

	<div use:melt={$accordion_root}>
		<h2 use:melt={$heading} class="text-4xl font-bold p-3 pb-5">Stages</h2>
		{#each stages as stage, i}
			<div
				use:melt={$item(stage.stage?.name ?? 'unnamed stage')}
				class="flex-1 my-3 rounded-lg pb-4 bg-bg-400"
			>
				<button
					use:melt={$trigger(stage.stage?.name ?? 'unnamed stage')}
					class="w-full text-3xl font-bold flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all"
				>
					{stage.stage?.name ?? 'unnamed stage'}
					{#if (stage.stage?.bestOf ?? -1) > 0}
						<h2 class="flex-1 text-right">Best of {stage.stage?.bestOf ?? -1}</h2>
					{/if}
				</button>

				{#if $value === stage.stage?.name ?? 'unnamed stage'}
					<div
						use:melt={$content(stage.stage?.name ?? 'unnamed stage')}
						transition:slide={{ duration: 200 }}
					>
						<StageCard
							tournamentId={tournament.key?.id ?? -1}
							stage={{
								name: stage.stage?.name ?? 'unnamed stage',
								bestOf: stage.stage?.bestOf ?? -1,
								stageOrder: i
							}}
						/>
					</div>
				{/if}
			</div>
		{:else}
			<div class="p-2 min-w-full">No stages found</div>
		{/each}
	</div>
</div>

<style>
</style>
