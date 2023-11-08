<script lang="ts">
	import type { Country, CountryList, GetTournamentResponse, RankRange, Tournament } from '$lib/api/tournaments';
	import Flag from './Flag.svelte';

	export let tournament: Tournament;
	export let rankRanges: RankRange[];
	export let countryRestrictions: CountryList;

	let countries: Country[] = countryRestrictions.countries!;
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
			<span>{rankRanges[0].max}</span>
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
	<div class="infoContent">
		{#if tournament.format > 0}
			 {tournament.format}v{tournament.format}
		{/if}
	</div>

	<!-- Country Restrictions -->
	{#if countries.length > 0 }
		<div class="infoHeading">Country Restrictions</div>
		<div class="infoContent">
			{#each countries as country}
				<Flag country={country.name.toLowerCase()} />
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
		@apply text-2xl font-bold text-left pr-10;
	}

	.infoContent {
		@apply text-xl pl-10 text-left;
	}

	@media (min-width: 1024px) {
		.infoHeading {
			text-align: right;
		}

		.infoContent {
			text-align: left;
		}
	}
</style>
