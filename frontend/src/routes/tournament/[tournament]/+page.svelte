<script lang="ts">
	import type { GetAllStagesResponse, Stage } from '$lib/api/stages';
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
	let tournament: Tournament = data.tournament;
	let rankRanges: RankRange[] = data.rankRanges;
	let countryRestrictions: CountryList = data.countryRestrictions!;
	let stages: Stage[] = data.stages;

	const {
		elements: { root: accordion_root, content, item, heading, trigger },
		states: { value }
	} = createAccordion({
		forceVisible: true
	});
</script>

<div class="bg-bg rounded-xl flex flex-col justify-center gap-8">
	<img
		src="https://i.ppy.sh/c654ce3b0a9aa87b1da2526a46141cf723c47935/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4f57432f323032332f696d672f6f7763323032332d62616e6e65722e6a7067"
		class="banner w-full h-2/3 opacity-60 img-grad object-cover"
		alt="banner for tournament '{tournament.name}'"
	/>

	<div class="lg:w-3/5 m-auto z-10">
		<TournamentInfo {tournament} {rankRanges} {countryRestrictions} />

		<hr class="py-5" />

		<div use:melt={$accordion_root}>
			<h2 use:melt={$heading} class="text-4xl font-bold p-3 pb-5">Stages</h2>
			{#each stages as stage, i}
				<div
					use:melt={$item(stage.name ?? 'unnamed stage')}
					class="flex-1 my-3 rounded-lg pb-4 bg-bg-400"
				>
					<button
						use:melt={$trigger(stage.name ?? 'unnamed stage')}
						class="w-full text-3xl font-bold flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all"
					>
						{stage.name ?? 'unnamed stage'}
						{#if (stage.bestOf ?? -1) > 0}
							<h2 class="flex-1 text-right">Best of {stage.bestOf ?? -1}</h2>
						{/if}
					</button>

					{#if $value === stage.name ?? 'unnamed stage'}
						<div
							use:melt={$content(stage.name ?? 'unnamed stage')}
							transition:slide={{ duration: 200 }}
						>
							<StageCard
								tournamentId={tournament.key?.id ?? -1}
								stage={{
									name: stage.name ?? 'unnamed stage',
									bestOf: stage.bestOf ?? -1,
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
</div>

<style>
	.img-grad {
		mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1), rgba(0, 0, 0, 0));
		mask-mode: auto;
		margin-bottom: -15%;
	}
</style>
