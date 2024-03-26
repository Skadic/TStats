import {
	TournamentServiceDefinition,
	type TournamentServiceClient,
	GetAllTournamentsResponse
} from '$lib/api/tournaments';
import { tstatsClient } from '$lib/rpc';

export async function load({ fetch, params }) {
	const client: TournamentServiceClient = tstatsClient(TournamentServiceDefinition);

	let tournaments: GetAllTournamentsResponse[] = [];

	for await (const tournament of client.getAll({})) {
		tournaments.push(tournament);
	}

	return {
		tournaments
	};
}
