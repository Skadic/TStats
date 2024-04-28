<script lang="ts">
	import type { GetAllTournamentsResponse, RangeList, Tournament } from '$lib/api/tournaments';

	export let tournament: Tournament;
	export let rankRestriction: RangeList;

  console.log(rankRestriction)

	let rankRange: String;
	switch (rankRestriction.ranges.length) {
		case 0:
			rankRange = 'Open Rank';
			break;
		case 1:
			rankRange = rankRestriction.ranges[0].min + ' - ' + rankRestriction.ranges[0].max;
			break;
		default:
			rankRange =
				'Tiered ' +
				rankRestriction.ranges[0].min +
				' - ' +
				rankRestriction.ranges[rankRestriction.ranges.length - 1].max;
	}
</script>

<div
	class="bg-bg-400 shadow-md shadow-bg-100 text-gray-200 rounded-2xl lg:rounded-lg min-w-full min-h-full container"
>
	<a href="/tournament/{tournament.key?.id}">
		<img
			class="object-cover rounded-t-xl lg:rounded-t-lg w-full lg:h-32"
			alt="banner"
			src="https://i.ppy.sh/f49c7b14d7308ee720c1dbd9b6c9d78dd1469a2d/68747470733a2f2f692e6962622e636f2f7a5a6a705076372f6d72656b6b2d6772616e646d61737465722d6375702d62616e6e65722d424554412d4c4f474f2e706e67"
		/>
		<div class="p-4">
			<h1 class="text-4xl lg:text-3xl font-bold text-center pb-2">{tournament.name}</h1>
			<div class="p-2 lg:p-0">
				<div class="text-right text-lg lg:text-lg truncate">
					{rankRange}
					{#if rankRange != 'Open Rank'}
						{tournament.bws ? 'with BWS' : 'without BWS'}
					{/if}
				</div>
				<div class="text-right text-lg truncate">{tournament.format}v{tournament.format}</div>
			</div>
		</div>
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
