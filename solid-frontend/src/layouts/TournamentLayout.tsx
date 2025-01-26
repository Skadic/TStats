import { LayoutComponent } from "../lib/types";

import styles from "./TournamentLayout.module.css";
import { tstatsClient } from "../lib/rpc";
import { TournamentContext } from "../contexts/TournamentContext";
import { getTournamentById } from "../lib/tournament";
import { createResource } from "solid-js";

export const TournamentLayout: LayoutComponent = (props) => {
	const id = () => Number.parseInt(props.params.id);

	// <TournamentInfo {tournament} {rankRanges} {countryRestrictions} />
	const client = tstatsClient();

	const [tournament] = createResource(id, (id: number) =>
		getTournamentById(id, client),
	);

	return (
		<TournamentContext.Provider value={tournament}>
			<div class="bg-bg rounded-xl flex flex-col justify-center gap-8">
				<img
					src="https://i.ppy.sh/c654ce3b0a9aa87b1da2526a46141cf723c47935/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4f57432f323032332f696d672f6f7763323032332d62616e6e65722e6a7067"
					class={`${styles.imgGrad} banner w-full h-2/3 opacity-60 object-cover`}
					alt="banner for tournament '{tournament.name}'"
				/>

				<div class="lg:w-3/5 m-auto z-10">
					{/* Tournament Info */}
					{props.children}
				</div>
			</div>
		</TournamentContext.Provider>
	);
};
