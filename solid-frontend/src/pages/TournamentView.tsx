import { createResource, For, Index, Match, Show, Switch, useContext } from "solid-js";
import { TournamentContext } from "../contexts/TournamentContext";
import TournamentInfo from "../components/TournamentInfo";

import { PageComponent } from "../lib/types";
import { Accordion } from "@kobalte/core/accordion";
import { getAllStages } from "../lib/stage";
import { getPool } from "../lib/pool";
import { Pool, Stage } from "../lib/api/v1";
import { StageCard } from "../components/StageCard";

export const TournamentView: PageComponent = () => {
	const tournamentContext = useContext(TournamentContext);
	if (!tournamentContext) {
		console.error("No tournament context in tournament view");
		return <>Tournament not found</>
	}

	const [stages] = createResource(tournamentContext, async (tournament) => {
		const stages = await getAllStages(tournament.id)
		const pools = await Promise.all(stages.map(stage => getPool(stage.tournamentId, stage.stageOrder)))

		const stagesWithPool: { stage: Stage, pool: Pool }[] = [];
		for (let i = 0; i < stages.length; i++) {
			stagesWithPool.push({ stage: stages[i], pool: pools[i] })
		}
		return stagesWithPool
	});

	function bestOf(n?: number) {
		if ((n ?? 0) <= 0) {
			return <></>
		} else {
			return <h2 class="flex-1 text-right">Best of {n}</h2>
		}
	};

	return (
		<Switch>
			<Match when={!tournamentContext()}>Tournament not found</Match>
			<Match when={tournamentContext()}>
				{(tournament) =>
					<div class="rounded-xl flex flex-col justify-center gap-8">
						<TournamentInfo tournament={tournament()} />
						<div class="lg:w-3/5 m-auto z-10">
							<hr class="py-5" />

							<Show when={stages()} fallback={<div class="p-2 min-w-full">Could not fetch stages</div>}>
								{(stages) =>
									<Accordion collapsible>
										<Index each={stages()} fallback={<div class="p-2 min-w-full">No stages found</div>}>
											{(stage, i) =>
												<Accordion.Item value={`stage-${i}`} class="flex-1 my-3 rounded-lg pb-4 bg-bg-400">
													<Accordion.Header class="text-4xl font-bold p-3 pb-5">
														<Accordion.Trigger class="w-full text-3xl font-bold flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all">
															{stage().stage.name}
															{bestOf(stage().stage.bestOf)}
														</Accordion.Trigger>
													</Accordion.Header>
													<Accordion.Content>
														<StageCard stage={stage().stage} pool={stage().pool} />
													</Accordion.Content>
												</Accordion.Item>
											}
										</Index>
									</Accordion>
								}
							</Show>
						</div>
					</div>
				}
			</Match>
		</Switch>
	);
};
