import { WebsocketTransport, createChannel, createClient } from 'nice-grpc-web'
import {
	TournamentServiceDefinition,
	type TournamentServiceClient,
	GetAllTournamentsResponse
} from '$lib/api/tournaments'

export async function load({ fetch, params }) {
	const channel = createChannel('http://0.0.0.0:9900')

	const client: TournamentServiceClient = createClient(TournamentServiceDefinition, channel)

	let tournaments: GetAllTournamentsResponse[] = []

	for await (const tournament of client.getAll({})) {
		tournaments.push(tournament)
	}

	return {
		channel,
		tournaments
	}
}
