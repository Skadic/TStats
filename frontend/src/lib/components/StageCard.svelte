<script lang="ts">
	import '$lib/styles/app.css';
	import { getAllStages } from '../ts/Stage';
	import type { Stage, Tournament } from '../ts/Types';

	export let stage: Stage;

	

</script>

<div class="scard">
	<a href="/{stage.tournament}/{stage.stage_number}" class="block w-full h-full"> {stage.name}</a>
</div>

<style>
	.scard {
		@apply bg-primary w-full h-16 rounded-xl flex font-nunito text-tx p-1 pl-2 mb-2 mr-2;
	}
</style>