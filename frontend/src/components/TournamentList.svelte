<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import TournamentCard from './TournamentCard.svelte';
	import '../styles/app.css';

	export let tournaments: Tournament[];
</script>

<div class="flex">
	<div class="flex flex-wrap justify-around lg:grid lg:grid-cols-3 lg:grid-flow-row gap-4">
		{#each tournaments as tournament}
			<div class="flex-grow-0 flex-shrink-0 basis-0 min-w-full lg:min-w-1/3 lg:justify-between">
				<TournamentCard {tournament} />
			</div>
		{/each}
	</div>
</div>

<style>
</style>
