<script lang="ts">
	import { PoolServiceDefinition, type PoolBracket, type PoolServiceClient } from '$lib/api/pool';
	import type { Stage } from '$lib/api/stages';
	import { tstatsClient } from '$lib/ts/rpc';
	import PoolList from './PoolList.svelte';
	import { slide } from 'svelte/transition';

	export let tournamentId: number;
	export let stage: Stage;

	async function fetchPool(tournament: number, stage: number) {
		const poolClient: PoolServiceClient = tstatsClient(PoolServiceDefinition);
		return await poolClient.get({
			stageKey: {
				stageOrder: stage,
				tournamentKey: { id: tournament }
			}
		});
	}

</script>

<div class="">
	{#await fetchPool(tournamentId, stage.stageOrder)}
		fetching pool...
	{:then pool}
		<div in:slide class="px-4 py-2">
			<PoolList {tournamentId} stage={stage.stageOrder} brackets={pool.pool?.brackets ?? []} />
		</div>
	{/await}
</div>
