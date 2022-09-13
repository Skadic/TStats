<script lang="ts">
	import '$lib/styles/app.css';
	import { getAllStages } from '$lib/ts/Stage';
	import type { Tournament } from '$lib/ts/Types';
	import StageCard from './StageCard.svelte';

	export let tournament: Tournament;

	let stagePromise = getAllStages(tournament.id);
</script>

<div class="sview-container">
	{#await stagePromise}
		<p>...waiting</p>
	{:then stages}
		{#each stages as stage}
			<StageCard {stage} />
		{/each}
	{:catch error}
		<p style="color: red">{error.message}</p>
	{/await}
</div>

<style>
	.sview-container {
		@apply w-auto overflow-y-auto m-2 grid grid-cols-2 gap-2;
	}
</style>
