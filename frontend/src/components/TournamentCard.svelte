<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import { formatRankRange, formatTournamentFormat } from '$lib/Tournament';

	export let tournament: Tournament;
	let rankRange = formatRankRange(tournament);
</script>

<div class="tournamentCard">
	<a href="/tournament/{tournament.id}">
		<img id="banner" alt="banner" src="https://i.ppy.sh/f49c7b14d7308ee720c1dbd9b6c9d78dd1469a2d/68747470733a2f2f692e6962622e636f2f7a5a6a705076372f6d72656b6b2d6772616e646d61737465722d6375702d62616e6e65722d424554412d4c4f474f2e706e67"/>
		<h1>{tournament.name}</h1>
		<h2>
			<span id="rankRange">
				{rankRange}
				{#if rankRange != 'Open Rank'}
					{tournament.bws ? 'with BWS' : 'without BWS'}
				{/if}
			</span>
			<span id="tournamentFormat">{formatTournamentFormat(tournament)}</span>
		</h2>
	</a>
</div>

<style>
	h1 {
		@apply text-3xl font-bold;
	}

	.tournamentCard {
		@apply text-ctp-base rounded-lg p-2 bg-gradient-to-br from-ctp-pink to-ctp-mauve min-w-full min-h-full;
	}

	#tournamentFormat {
		@apply float-right;
	}

	#banner {
		height: 100px;
		object-fit: cover;
	}
</style>
