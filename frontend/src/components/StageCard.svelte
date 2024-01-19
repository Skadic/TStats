<script lang="ts">
	import { PoolServiceDefinition, type PoolBracket, type PoolServiceClient } from '$lib/api/pool';
	import type { Stage } from '$lib/api/stages';
	import { createChannel, createClient } from 'nice-grpc-web';
	import PoolList from './PoolList.svelte';

	export let tournamentId: number;
	export let stage: Stage;

	$: poolPromise = (async function fetchPool(tournament: number, stage: number) {
		const channel = createChannel('http://0.0.0.0:3000');
		const poolClient: PoolServiceClient = createClient(PoolServiceDefinition, channel);
		return await poolClient.get({
			stageKey: {
				stageOrder: stage,
				tournamentKey: { id: tournament }
			}
		});
	})(tournamentId, stage.stageOrder);
</script>

<div class="">
	{#await poolPromise}
		fetching pool...
	{:then pool}
		<div class="px-4 py-2">
			<PoolList brackets={pool.pool?.brackets ?? []} />
		</div>
	{/await}
</div>
