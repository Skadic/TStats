<script lang="ts">
	import '$lib/styles/app.css';
	import { getAllStages } from '../ts/Stage';
	import type { Stage, Tournament } from '../ts/Types';

	export let stage: Stage;

	

</script>

<div class="scard">
	<a href="/{stage.tournament_id}/{stage.idx}" class="w-full h-full flex">
		<div class="text-6xl">{stage.stage_name}</div>
		<div class="w-full text-right text-2xl">Best of {stage.best_of}</div>
	</a>

</div>

<style>
	.scard {
		@apply bg-gradient-to-tr from-primary to-secondary w-full h-16 rounded-xl flex font-nunito text-tx p-1 pl-2 mb-2 mr-2;
	}
</style>