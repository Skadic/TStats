import { StageServiceDefinition, type StageServiceClient, Stage } from '$lib/api/stages';
import { tstatsClient } from '$lib/ts/rpc';

export async function load({ params, parent }: any) {
	let parent_data = await parent();
	const stageClient: StageServiceClient = tstatsClient(StageServiceDefinition, parent_data.channel);
	const request = stageClient.getAll({
		tournamentKey: { id: parseInt(params.tournament) }
	});

	const stages: Stage[] = [];
	for await (const stage of request) {
		stages.push(stage?.stage!);
	}

	return {
		stages: stages
	};
}
