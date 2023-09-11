<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import { formatRankRange, formatTournamentFormat } from '$lib/Tournament';

	export let tournament: Tournament;
	let rankRange = formatRankRange(tournament);
</script>

<div
	class="bg-bg-400 shadow-md shadow-bg-100 text-gray-200 rounded-2xl lg:rounded-lg min-w-full min-h-full container"
>
	<a href="/tournament/{tournament.id}">
		<img
			class="object-cover rounded-t-xl lg:rounded-t-lg w-full lg:h-32"
			alt="banner"
			src="https://i.ppy.sh/f49c7b14d7308ee720c1dbd9b6c9d78dd1469a2d/68747470733a2f2f692e6962622e636f2f7a5a6a705076372f6d72656b6b2d6772616e646d61737465722d6375702d62616e6e65722d424554412d4c4f474f2e706e67"
		/>
		<h1 class="text-5xl lg:text-3xl font-bold text-center m-2">{tournament.name}</h1>
		<div class="text-xl lg:text-lg">
			{rankRange}
			{#if rankRange != 'Open Rank'}
				{tournament.bws ? 'with BWS' : 'without BWS'}
			{/if}
		</div>
		<div class="float-right text-xl lg:text-lg">{formatTournamentFormat(tournament)}</div>
	</a>
</div>



<style>
	.container {
		transition: all;
		transition-duration: 0.2s;
	}
	.container:hover {
		transform: scale(1.03);
	}
</style>