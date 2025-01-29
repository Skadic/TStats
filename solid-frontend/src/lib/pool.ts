import { Pool } from "./api/v1";
import { tstatsClient, TStatsClient } from "./rpc";

export async function getPool(
	tournamentId: number,
    stageOrder: number,
	client: TStatsClient = tstatsClient(),
): Promise<Pool> {
	 return await client
		.GET("/api/tournament/{tournament_id}/stage/{stage_order}/pool", {
			params: {
				path: {
					tournament_id: tournamentId,
                    stage_order: stageOrder
				},
			},
		})
		.then(
			(res) => {
				const pool = res.data;
				if (pool) {
					return pool;
				}
				throw new Error(`could not fetch pool for tournament with id ${tournamentId} and stage ${stageOrder}: ${res.error}`);
			},
			(err) => {
				throw new Error(`could not fetch pool for tournament with id ${tournamentId} and stage ${stageOrder}: ${err}`);
			},
		);
}
