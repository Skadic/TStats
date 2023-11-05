<script lang="ts">
	import TournamentCard from './TournamentCard.svelte';
	import '../styles/app.css';
	import { Tournament, type GetAllTournamentsResponse, RangeList } from '$lib/api/tournaments';

	export let tournaments: GetAllTournamentsResponse[];
</script>

<div class="flex flex-wrap justify-around flex-col lg:flex-row gap-y-8 gap-x-4">
	{#each tournaments as tournament}
		<div class="flex-grow-1 w-full lg:min-w-1/3 lg:justify-between lg:w-20">
			<TournamentCard
				tournament={tournament.tournament ?? Tournament.fromPartial({})}
				rankRestriction={tournament.rankRestrictions ?? RangeList.fromPartial({})}
			/>
		</div>
	{/each}
</div>

<style>
</style>
