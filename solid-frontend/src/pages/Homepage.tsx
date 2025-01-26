import { EmptyObject, PageComponent } from "../lib/types";
import ImportantTournaments from "../components/ImportantTournaments";
import { tstatsClient } from "../lib/rpc";
import { createResource } from "solid-js";
import { Tournament } from "../lib/api/v1types";
import TournamentList from "../components/TournamentList";

async function fetchTournaments(): Promise<Tournament[]> {
	const client = tstatsClient();
	return await client.GET("/tournament").then((result) => result.data!);
}

export const Homepage: PageComponent<EmptyObject> = () => {
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
