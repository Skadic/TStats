import { RouteSectionProps } from "@solidjs/router";
import { Component, EmptyObject } from "../lib/types";
import ImportantTournaments from "../components/ImportantTournaments";
import { tstatsClient } from "../lib/rpc";
import { createResource } from "solid-js";
import { Tournament } from "../lib/api/v1types";
import TournamentList from "../components/TournamentList";

async function fetchTournaments(): Promise<Tournament[]> {
	const client = tstatsClient();
	return await client.GET("/tournaments").then((result) => result.data!);
}

const TournamentView: Component<RouteSectionProps<EmptyObject>> = () => {
	const [tournaments] = createResource(fetchTournaments);

	return (
		<div class="lg:w-3/5 m-auto">
			<div class="py-2"></div>
			<div class="px-6 py-4">
				<ImportantTournaments />
			</div>
			<hr class="py-4" />
			<div class="px-1">
				<TournamentList tournaments={tournaments()} />
			</div>
		</div>
	);
};

export default TournamentView;
