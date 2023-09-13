import { ExtendedTournament } from '$lib/Tournament.js';

export type TournamentResult = {
	tournament: ExtendedTournament;
};

export async function load({ fetch, params }): Promise<TournamentResult> {
	const res = await fetch(
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

	const js = await res.json();
	//console.log(js);
	const tournament: ExtendedTournament = ExtendedTournament.deserialize(js);

	return {
		tournament
	};
}
