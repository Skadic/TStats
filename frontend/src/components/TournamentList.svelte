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
		@apply p-2;
		flex: 0 1;
		min-width: 95%;
	}

	#tournamentList {
		display: flex;
		flex-wrap: wrap;
		justify-content: space-around;
	}

	@media screen and (min-width: 80rem) {
		.tournamentCard {
			min-width: 33%;
		}

		#tournamentList {
			justify-content: space-between;
		}
	}

	#container {
		@apply flex;
	}
</style>
