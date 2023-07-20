<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import TournamentCard from './TournamentCard.svelte';
	import '../styles/app.css';

	export let tournaments: Tournament[];
</script>

<div class="tournamentList">
	{#each tournaments as tournament}
		<div class="tournamentCard">
			<TournamentCard {tournament} />
		</div>
	{/each}
</div>

<style>
	.tournamentCard {
		@apply m-2;
	}
    .tournamentList {
        @apply grid grid-cols-3;
    }
</style>
