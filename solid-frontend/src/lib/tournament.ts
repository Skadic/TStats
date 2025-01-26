import { type TStatsClient, tstatsClient } from "./rpc";
import type { Tournament } from "./api/v1types";

export async function getTournamentById(
	id: number,
	client: TStatsClient = tstatsClient(),
): Promise<Tournament> {
	console.log(`fetching tournament with id ${id}`)
	return await client
		.GET("/api/tournament/{id}", {
			params: {
				path: {
					id,
				},
			},
		})
		.then(
			(res) => {
				const tournament = res.data;
				if (tournament) {
					return tournament;
				}
				throw new Error(`could not fetch tournament with id ${id}: ${res.error}`);
			},
			(err) => {
				throw new Error(`could not fetch tournament with id ${id}: ${err}`);
			},
		);
}
