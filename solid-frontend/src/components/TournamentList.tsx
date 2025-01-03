import { For } from "solid-js";
import { Tournament } from "../lib/api/v1types";
import { Component } from "../lib/types";
import TournamentCard from "./TournamentCard";

const TournamentList: Component<{ tournaments: Tournament[] | undefined }> = (
	props,
) => {
	const tournaments = () => props.tournaments;
	return (
		<div class="flex flex-wrap justify-around flex-col lg:flex-row gap-y-8 gap-x-4">
			<For each={tournaments()}>
				{(tournament) => (
					<div class="flex-grow-1 w-full lg:min-w-1/3 lg:justify-between lg:w-20">
						<TournamentCard tournament={tournament} />
					</div>
				)}
			</For>
		</div>
	);
};

export default TournamentList;
