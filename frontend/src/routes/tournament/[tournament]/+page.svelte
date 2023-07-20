<script lang="ts">
	import type { Stage } from '$lib/Stage';
	import {
		formatRankRangeDetailed,
		formatTournamentFormat,
		type ExtendedTournament
	} from '$lib/Tournament';
	import Flag from '../../../components/Flag.svelte';
	import StageCard from '../../../components/StageCard.svelte';

	export let data;
	let tournament: ExtendedTournament = data.tournament;
	let stages: Stage[] = data.stages;
	stages.sort((a: Stage, b: Stage) => a.order - b.order);
	let rankRanges = formatRankRangeDetailed(tournament);
</script>

<h1>{tournament.name}</h1>
<div class="container">
	<div class="tournamentInfo">
		<!-- Rank Ranges -->
		<div class="infoHeading">
			Rank Range{#if rankRanges.length > 1}s{/if}
		</div>
		<div class="infoContent">
			{#if rankRanges[0] == 'Open Rank'}
				Open Rank
			{:else if rankRanges.length == 1}
				<span>{rankRanges[0].split('-')[0]}</span>
				<span class="px-1 m-0">-</span>
				<span>{rankRanges[0].split('-')[1]}</span>
			{:else}
				<table class="min-w-full">
					{#each rankRanges as range}
						<tr>
							<td class="font-bold">{range.split(':')[0]}</td>
							<td class="pl-3">{range.split(':')[1].split('-')[0]}</td>
							<td class="px-1 m-0">-</td>
							<td>{range.split(':')[1].split('-')[1]}</td>
						</tr>
					{/each}
				</table>
			{/if}
		</div>

		<!-- BWS -->
		{#if rankRanges[0] != 'Open Rank'}
			<div class="infoHeading">BWS</div>
			<div class="infoContent">{tournament.bws ? 'Yes' : 'No'}</div>
		{/if}

		<!-- Format -->
		<div class="infoHeading">Match Format</div>
		<div class="infoContent">{formatTournamentFormat(tournament)}</div>

		<!-- Country Restrictions -->
		{#if tournament.country_restriction !== null && tournament.country_restriction.length > 0}
			<div class="infoHeading">Country Restrictions</div>
			<div class="infoContent">
				{#each tournament.country_restriction as country}
					<Flag country={country.toLowerCase()} />
				{/each}
			</div>
		{/if}
	</div>
	<div class="stageList">
		{#each stages as stage}
			<div class="stageCard">
				<StageCard {stage} />
			</div>
		{:else}
			<div class="stageCard">No stages found</div>
		{/each}
	</div>
</div>

<style>
	h1 {
		@apply text-4xl font-bold text-center p-3 mb-5;
	}

	.container {
		@apply grid grid-cols-2 min-w-full p-3 px-6 bg-ctp-mantle rounded-xl;
		grid-template-columns: 60% auto;
	}

	.tournamentInfo {
		@apply grid grid-cols-2 min-w-full;
		grid-template-columns: 40% max-content;
		grid-auto-rows: min-content;
	}

	.tournamentInfo > * {
		@apply py-3;
	}

	.infoHeading {
		@apply flex text-2xl font-bold items-center justify-center;
	}

	.infoContent {
		@apply flex flex-grow-0 text-xl items-center justify-center;
	}

	.stageList {
		@apply min-w-full;
	}

	.stageCard {
		@apply m-2 min-w-full;
	}

	.infoContent {
		@apply mx-7;
	}
</style>
