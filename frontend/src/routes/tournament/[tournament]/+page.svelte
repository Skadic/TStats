<script lang="ts">
	import type { Stage } from '$lib/api/stages';
	import type { Tournament } from '$lib/api/tournaments';
	import { createAccordion, melt } from '@melt-ui/svelte';
	import { slide } from 'svelte/transition';
	import StageCard from '$lib/components/StageCard.svelte';

	export let data;
	let tournament: Tournament = data.tournament;
	let stages: Stage[] = data.stages;

	const {
		elements: { root: accordion_root, content, item, heading, trigger },
		states: { value }
	} = createAccordion({
		forceVisible: true
	});
</script>

<div use:melt={$accordion_root}>
	<h2 use:melt={$heading} class="text-4xl font-bold p-3 pb-5">Stages</h2>
	{#each stages as stage, i}
		<div
			use:melt={$item(stage.name ?? 'unnamed stage')}
			class="flex-1 my-3 rounded-lg pb-4 bg-bg-400"
		>
			<button
				use:melt={$trigger(stage.name ?? 'unnamed stage')}
				class="w-full text-3xl font-bold flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all"
			>
				{stage.name ?? 'unnamed stage'}
				{#if (stage.bestOf ?? -1) > 0}
					<h2 class="flex-1 text-right">Best of {stage.bestOf ?? -1}</h2>
				{/if}
			</button>

			{#if $value === stage.name ?? 'unnamed stage'}
				<div
					use:melt={$content(stage.name ?? 'unnamed stage')}
					transition:slide={{ duration: 200 }}
				>
					<StageCard
						tournamentId={tournament.key?.id ?? -1}
						stage={{
							name: stage.name ?? 'unnamed stage',
							bestOf: stage.bestOf ?? -1,
							stageOrder: i
						}}
					/>
				</div>
			{/if}
		</div>
	{:else}
		<div class="p-2 min-w-full">No stages found</div>
	{/each}
</div>

<style>
</style>
