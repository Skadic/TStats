import { Match, Resource, Show, Switch, useContext } from "solid-js";
import { Component } from "../lib/types";
import { TournamentContext } from "../contexts/TournamentContext";
import TournamentInfo from "../components/TournamentInfo";

import styles from "./TournamentView.module.css";

const TournamentView: Component = () => {
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
							<div class="bg-bg rounded-xl flex flex-col justify-center gap-8">
								<img
									src="https://i.ppy.sh/c654ce3b0a9aa87b1da2526a46141cf723c47935/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4f57432f323032332f696d672f6f7763323032332d62616e6e65722e6a7067"
									class={`banner w-full h-2/3 opacity-60 object-cover ${styles.imgGradient}`}
									style={""}
									alt="banner for tournament '{tournament.name}'"
								/>
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

export default TournamentView;
