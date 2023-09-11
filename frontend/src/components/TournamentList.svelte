<script lang="ts">
	import type { Tournament } from '$lib/Tournament';
	import TournamentCard from './TournamentCard.svelte';
	import '../styles/app.css';

	export let tournaments: Tournament[];
</script>

<div id="container">
	<div id="tournamentList">
		{#each tournaments as tournament}
			<div class="tournamentCard">
				<TournamentCard {tournament} />
			</div>
		{/each}
	</div>
</div>

<style>
	.tournamentCard {
		@apply m-2 flex-1;
	}
	#tournamentList {
		@apply flex flex-wrap;
	}

	#container {
		@apply flex justify-center;
	}

</style>
