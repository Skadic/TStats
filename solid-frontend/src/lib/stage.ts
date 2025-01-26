import { Stage } from "./api/v1";
import { tstatsClient, TStatsClient } from "./rpc";

export async function getAllStages(
	tournamentId: number,
	client: TStatsClient = tstatsClient(),
): Promise<Stage[]> {
	 return await client
		.GET("/api/tournament/{tournament_id}/stage", {
			params: {
				path: {
					tournament_id: tournamentId
				},
			},
		})
		.then(
			(res) => {
				const stages = res.data;
				if (stages) {
					return stages;
				}
				throw new Error(`could not fetch stages for tournament with id ${tournamentId}: ${res.error}`);
			},
			(err) => {
				throw new Error(`could not fetch stages for tournament with id ${tournamentId}: ${err}`);
			},
		);
}
