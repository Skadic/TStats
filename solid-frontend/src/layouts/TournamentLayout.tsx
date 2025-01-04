import { createResource, Match, Suspense, Switch } from "solid-js";
import TournamentInfo from "../components/TournamentInfo";
import { LayoutComponent } from "../lib/types";
import { tstatsClient } from "../lib/rpc";

const TournamentLayout: LayoutComponent = (props) => {
	console.log(JSON.stringify(props));
	const id = () => Number.parseInt(props.params.id);
	const [tournamentResult] = createResource(id, (id) =>
		tstatsClient()
			.GET("/tournaments/{id}", {
				params: {
					path: {
						id,
					},
				},
			})
			.then((res) => {
				console.log(JSON.stringify(res));
				return res;
			}),
	);

	return (
		<div class="bg-bg rounded-xl flex flex-col justify-center gap-8">
			<img
				src="https://i.ppy.sh/c654ce3b0a9aa87b1da2526a46141cf723c47935/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4f57432f323032332f696d672f6f7763323032332d62616e6e65722e6a7067"
				class="banner w-full h-2/3 opacity-60 img-grad object-cover"
				alt="banner for tournament '{tournament.name}'"
			/>

			<Switch fallback={<div>Loading...</div>}>
				<Match when={tournamentResult()?.error}>Error loading Tournament</Match>
				<Match when={tournamentResult()?.data}>
					<TournamentInfo tournament={tournamentResult()?.data!} />
					<div class="lg:w-3/5 m-auto z-10">
						<hr class="py-5" />
						{props.children}
					</div>
				</Match>
			</Switch>
		</div>
	);
};

export default TournamentLayout;
