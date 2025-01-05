import { createResource } from "solid-js";
import { LayoutComponent } from "../lib/types";
import { tstatsClient } from "../lib/rpc";
import { getTournamentById } from "../lib/tournament";
import { TournamentContext } from "../contexts/TournamentContext";

const TournamentContextProvider: LayoutComponent = (props) => {
	const id = () => Number.parseInt(props.params.id);
	const client = tstatsClient();

	const [tournament] = createResource(id, (id) =>
		getTournamentById(id, client),
	);

	return (
		<TournamentContext.Provider value={tournament}>
			{props.children}
		</TournamentContext.Provider>
	);
};

export default TournamentContextProvider;
