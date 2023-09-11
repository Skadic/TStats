<script lang="ts">
	import { formatRankRangeDetailed, formatTournamentFormat } from '$lib/Tournament';
	import type { TournamentResult } from '../routes/tournament/[tournament]/+page';
	import Flag from './Flag.svelte';

	export let data: TournamentResult;
	let tournament: TournamentResult = data;
	let rankRanges = formatRankRangeDetailed(tournament);
</script>

<div class="tournamentInfo flex flex-wrap w-3/4">
	<!-- Rank Ranges -->
	<div class="infoHeading">
		Rank Range{#if rankRanges.length > 1}s{/if}
	</div>
	<div class="infoContent">
		{#if rankRanges[0] == 'Open Rank'}
			Open Rank
		{:else if rankRanges.length == 2}
			<span>{rankRanges[0]}</span>
			<span class="px-1 m-0">-</span>
			<span>{rankRanges[1]}</span>
		{:else}
			<table class="min-w-full">
				{#each rankRanges as range}
					<tr>
						<td class="font-bold">{range.split(':')[0]}</td>
						<td class="pl-3">{range.split(':')[1].split('-')[0]}</td>
						<td class="px-1 m-0">-</td>
						<td class="">{range.split(':')[1].split('-')[1]}</td>
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
	{#if tournament.country_restrictions !== null && tournament.country_restrictions.length > 0}
		<div class="infoHeading">Country Restrictions</div>
		<div class="infoContent">
			{#each tournament.country_restrictions as country}
				<Flag country={country.toLowerCase()} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.tournamentInfo > * {
		@apply py-3;
		flex: 0 1 40%;
	}

	.infoHeading {
		@apply flex text-2xl font-bold justify-end items-center pr-10;
	}

	.infoContent {
		@apply flex flex-grow-0 text-xl items-center justify-start pl-10;
	}
</style>
