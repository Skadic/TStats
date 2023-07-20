<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import { formatRankRange, formatTournamentFormat } from '$lib/Tournament';

	export let tournament: Tournament;
	let rankRange = formatRankRange(tournament);
</script>

<div class="tournamentCard">
	<a href="/tournament/{tournament.id.split(':')[1]}">
		<h1>{tournament.name}</h1>
		<h2>
			<span class="rankRange">
				{rankRange}
				{#if rankRange != 'Open Rank'}
					{tournament.bws ? 'with BWS' : 'without BWS'}
				{/if}
			</span>
			<span class="tournamentFormat">{formatTournamentFormat(tournament)}</span>
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

	.tournamentFormat {
		@apply float-right;
	}
</style>
