import { PoolBracket } from '$lib/Pool.js';
import type { Stage } from '$lib/Stage';
import { ExtendedTournament } from '$lib/Tournament.js';

export type TournamentResult = {
	tournament: ExtendedTournament;
	poolBrackets: PoolBracket[][];
};

export async function load({ fetch, params }): Promise<TournamentResult> {
	const tournamentResult = await fetch(
		`http://0.0.0.0:3000/api/tournament?` +
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
	//console.log(js);
	const tournament: ExtendedTournament = ExtendedTournament.deserialize(
		await tournamentResult.json()
	);
	const numStages: number = tournament.stages.length;

	let poolPromises: Promise<PoolBracket[]>[] = [];

	for (let i = 0; i < numStages; i++) {
		const stage: Stage = tournament.stages[i];
		const res = fetch(
			`http://0.0.0.0:3000/api/pool?` +
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
