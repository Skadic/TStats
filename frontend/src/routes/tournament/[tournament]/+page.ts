import { PoolServiceDefinition, type PoolServiceClient, GetPoolResponse } from '$lib/api/pool.js';
import {
	StageServiceDefinition,
	type StageServiceClient,
	GetAllStagesResponse
} from '$lib/api/stages.js';
import { TournamentServiceDefinition, type TournamentServiceClient, GetTournamentResponse } from '$lib/api/tournaments';
import { createChannel, createClient } from 'nice-grpc-web';

export async function load({ fetch, params }) {
	const channel = createChannel('http://0.0.0.0:3000');

	const tournamentClient: TournamentServiceClient = createClient(
		TournamentServiceDefinition,
		channel
	);
	const stageClient: StageServiceClient = createClient(StageServiceDefinition, channel);
	const poolClient: PoolServiceClient = createClient(PoolServiceDefinition, channel);

	let tournament: GetTournamentResponse = await tournamentClient.get({
		key: { id: parseInt(params.tournament) }
	});

	let request = stageClient.getAll({
		tournamentKey: { id: parseInt(params.tournament) }
	});

	let stages: GetAllStagesResponse[] = [];
	for await (const stage of request) {
		stages.push(stage);
	}

	let poolResponse: GetPoolResponse = await poolClient.get({
		stageKey: {
			stageOrder: 0,
			tournamentKey: {
				id: 1
			}
		}
	});
  console.log(poolResponse.pool)

	return {
		tournament,
		channel,
		stages,
		pool: poolResponse.pool?.brackets
	};
}

/*
export async function load({ fetch, params }): Promise<TournamentResult> {
	const tournamentResult = await fetch(
		`http://172.31.26.242:3000/api/tournament?` +
			new URLSearchParams({
				id: params.tournament
			}),
		{
			method: 'GET',
			headers: new Headers({
				'Content-Type': 'application/json'
			})
		}
	);
	const tournament: ExtendedTournament = ExtendedTournament.deserialize(
		await tournamentResult.json()
	);
	const numStages: number = tournament.stages.length;

	let poolPromises: Promise<PoolBracket[]>[] = [];

	for (let i = 0; i < numStages; i++) {
		const stage: Stage = tournament.stages[i];
		const res = fetch(
			`http://172.31.26.242:3000/api/pool?` +
				new URLSearchParams({
					tournament_id: params.tournament,
					stage_order: stage.stageOrder.toString()
				}),
			{
				method: 'GET',
				headers: new Headers({
					'Content-Type': 'application/json'
				})
			}
		).then(async v => await v.json())
		
		poolPromises.push(res);
	}

	let poolBrackets: PoolBracket[][] = []
	for (let i = 0; i < numStages; i++) {
		let newArr = []
		const arr = await poolPromises[i];
		for (const bracket of arr) {
			newArr.push(PoolBracket.deserialize(bracket))
		}
		poolBrackets.push(newArr)
	}

	return {
		tournament,
		poolBrackets
	};
}
*/
