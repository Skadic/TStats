<script lang="ts">
	import type { ExtendedTournament, RankRange, Tournament } from '$lib/Tournament';
	import Flag from './Flag.svelte';

	export let data: ExtendedTournament;
	let extTournament: ExtendedTournament = data;
	let tournament: Tournament = data.tournament;
	let rankRanges: RankRange[] = extTournament.tournament.rank_range;
</script>

<div class="tournamentInfo flex flex-wrap w-3/4">
	<!-- Rank Ranges -->
	<div class="infoHeading">
		Rank Range{#if rankRanges.length > 1}s{/if}
	</div>
	<div class="infoContent">
		{#if rankRanges.length == 0}
			Open Rank
		{:else if rankRanges.length == 1}
			<span>{rankRanges[0].min}</span>
			<span class="px-1">-</span>
			<span>{rankRanges[1].max}</span>
		{:else}
			<table class="min-w-full">
				{#each rankRanges as range, i}
					<tr>
						<td class="font-bold">Tier {i + 1}:</td>
						<td class="pl-3">{range.min}</td>
						<td class="px-1">-</td>
						<td class="">{range.max}</td>
					</tr>
				{/each}
			</table>
		{/if}
	</div>
	<!-- BWS -->
	{#if rankRanges.length == 0}
		<div class="infoHeading">BWS</div>
		<div class="infoContent">{tournament.bws ? 'Yes' : 'No'}</div>
	{/if}

	<!-- Format -->
	<div class="infoHeading">Match Format</div>
	<div class="infoContent">{tournament.formatTournamentFormat()}</div>

	<!-- Country Restrictions -->
	{#if extTournament.countryRestrictions !== null && extTournament.countryRestrictions.length > 0}
		<div class="infoHeading">Country Restrictions</div>
		<div class="infoContent">
			{#each extTournament.countryRestrictions as country}
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
