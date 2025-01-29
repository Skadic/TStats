import { For, Index, Show } from "solid-js";
import { Pool, Stage } from "../lib/api/v1";
import { Component } from "../lib/types";

import { Accordion } from "@kobalte/core/accordion";
import { PoolMapCard } from "./PoolMapCard";


export const StageCard: Component<{ stage: Stage, pool: Pool }> = (props) => {
	const stage = props.stage;
	const pool = props.pool;

	return <Index each={pool.brackets}>
		{(bracket, bracketIndex) =>
			<div class="p-2">
				<div class="flex flex-col gap-2 px-4 bg-bg-500 shadow-xl rounded-xl py-4 pb-6">
					<Index each={bracket().maps}>
						{(map, mapIndex) =>
							<div class=" transition-all duration-200 hover:scale-105">
								<PoolMapCard map={map()} tournamentId={stage.tournamentId} stageOrder={stage.stageOrder} bracket={bracket()} mapOrder={mapIndex}/>
							</div>
						}
					</Index>
				</div>
			</div>
		}
	</Index>;
};
