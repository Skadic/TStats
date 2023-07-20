<script lang="ts">
	import type { Stage } from '$lib/Stage';
	export let stage: Stage;
</script>

<div class="stageCard">
	<a href="/stage/{stage.id.split(':')[1]}">
		<h1>{stage.name}</h1>
	</a>
</div>

<style>
	h1 {
		@apply text-3xl font-bold;
	}

	.stageCard {
		@apply text-ctp-base rounded-lg p-2 bg-gradient-to-br from-ctp-pink to-ctp-mauve;
	}
</style>
