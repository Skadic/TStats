import {
	StageServiceDefinition,
	type StageServiceClient,
	Stage
} from '$lib/api/stages';
import {
	TournamentServiceDefinition,
	type TournamentServiceClient,
	GetTournamentResponse
} from '$lib/api/tournaments';
import { tstatsChannel, tstatsClient } from '$lib/ts/rpc';

export async function load({ params }: any) {
	const channel = tstatsChannel();
	const key = { id: parseInt(params.tournament) };
	const tournamentClient: TournamentServiceClient = tstatsClient(
		TournamentServiceDefinition,
		channel
	);
	const tournamentResponse: GetTournamentResponse = await tournamentClient.get({
		key: key
	});

	return {
		tournament: tournamentResponse.tournament!,
		rankRanges: tournamentResponse.rankRestrictions?.ranges!,
		countryRestrictions: tournamentResponse.countryRestrictions!,
		channel,
	};
}