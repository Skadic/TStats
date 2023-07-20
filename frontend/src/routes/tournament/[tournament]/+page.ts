import type { Stage } from '$lib/Stage';
import type { ExtendedTournament } from '$lib/Tournament.js';

export async function load({ fetch, params }) {
	const res = await fetch(
		`http://127.0.0.1:3000/api/tournament?` +
			new URLSearchParams({
				id: params.tournament
			}),
		{
			method: 'GET',
			headers: new Headers({
				'Content-Type': 'application/json',
			})
		}
	);
	//console.log(await res.json())
	const tournament: ExtendedTournament = await res.json();

	return {
		tournament: tournament,
		stages: tournament.stages,
	};
}
