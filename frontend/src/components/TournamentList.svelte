<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import TournamentCard from './TournamentCard.svelte';
	import '../styles/app.css';

	export let tournaments: Tournament[];
</script>

<div class="flex flex-wrap justify-around flex-col lg:flex-row gap-y-8 gap-x-4">
	{#each tournaments as tournament}
		<div class="flex-grow-1 w-full lg:min-w-1/3 lg:justify-between lg:w-20">
			<TournamentCard {tournament} />
		</div>
	{/each}
</div>

<style>
</style>
