import { Match, Show, Switch, useContext } from "solid-js";
import { TournamentContext } from "../contexts/TournamentContext";
import TournamentInfo from "../components/TournamentInfo";

import { PageComponent } from "../lib/types";

export const TournamentView: PageComponent = () => {
	const tournamentContext = useContext(TournamentContext);
	if (!tournamentContext) {
		console.error("No tournament context in tournament view");
	}
	return (
		<Show
			when={tournamentContext}
			fallback={<div>Tournament not found</div>}
			keyed
		>
			{(tournamentResult) => (
				<Switch fallback={<div>Loading...</div>}>
					<Match when={tournamentResult() === null}>Tournament not found</Match>
					<Match when={tournamentResult()}>
						{(tournament) => (
							<div class="rounded-xl flex flex-col justify-center gap-8">
								<TournamentInfo tournament={tournament()} />
								<div class="lg:w-3/5 m-auto z-10">
									<hr class="py-5" />
									{/* Children */}
								</div>
							</div>
						)}
					</Match>
				</Switch>
			)}
		</Show>
	);
};
