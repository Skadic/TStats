import { type TStatsClient, tstatsClient } from "./rpc";
import type { Tournament } from "./api/v1types";

export async function getTournamentById(
	id: number,
	client: TStatsClient = tstatsClient(),
): Promise<Tournament | null> {
	 return await client
		.GET("/tournaments/{id}", {
			params: {
				path: {
					id,
				},
			},
		})
		.then(
			(res) => {
				const tournament = res.data;
				if (res.data) {
					return res.data;
				}
				console.error(`could not fetch tournament with id ${id}: ${res.error}`);
				return null;
			},
			(err) => {
				console.error(`could not fetch tournament with id ${id}: ${err}`);
				return null;
			},
		);
}
