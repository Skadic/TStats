/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import { Route, Router } from "@solidjs/router";
import App from "./App";
import Layout from "./layouts/Layout";
import { Homepage } from "./pages/Homepage";
import { AuthPage } from "./pages/AuthPage";
import { TournamentView } from "./pages/TournamentView";
import { TournamentLayout } from "./layouts/TournamentLayout";

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
	throw new Error(
		"Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?",
	);
}

render(
	() => (
		<Router root={Layout}>
			<Route path="/test" component={App} />
			<Route path="/auth" component={AuthPage} />
			<Route path="/" component={Homepage} />
			<Route path="/tournament/:id" component={TournamentLayout}>
				<Route path="/" component={TournamentView} />
			</Route>
		</Router>
	),
	root!,
);
