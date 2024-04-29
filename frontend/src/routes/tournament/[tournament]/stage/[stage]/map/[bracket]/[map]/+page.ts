import {
	GetScoresResponse,
	type ScoreServiceClient,
	ScoreServiceDefinition
} from '$lib/api/scores';
import {
	GetStageRequest,
	GetStageResponse,
	type StageServiceClient,
	StageServiceDefinition
} from '$lib/api/stages';
import { tstatsClient } from '$lib/ts/rpc';

export async function load({ params, parent }: any) {
	let parent_data = await parent();
	const stageClient: StageServiceClient = tstatsClient(StageServiceDefinition, parent_data.channel);
	const scoreClient: ScoreServiceClient = tstatsClient(ScoreServiceDefinition, parent_data.channel);
	const stageKey = {
		tournamentKey: {
			id: Number(params.tournament)
		},
		stageOrder: Number(params.stage)
	};
	const scoreRequest = scoreClient.get({
		poolMapKey: {
			bracketKey: {
				stageKey,
				bracketOrder: Number(params.bracket)
			},
			mapOrder: Number(params.map)
		}
	});

	const scoreResponse: GetScoresResponse = await scoreRequest;

	const stageRequest = stageClient.get({
		key: stageKey
	});
	const stageResponse: GetStageResponse = await stageRequest;

	return {
		beatmap: scoreResponse.beatmap!,
		scores: scoreResponse.scores,
		brackets: stageResponse.pool?.brackets!
	};
}
