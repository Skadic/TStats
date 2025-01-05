/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import { Route, Router } from "@solidjs/router";
import App from "./App";
import Layout from "./layouts/Layout";
import TournamentListView from "./pages/TournamentListView";
import AuthPage from "./pages/AuthPage";
import TournamentContextProvider from "./layouts/TournamentContextProvider";
import TournamentView from "./pages/TournamentView";

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
			<Route path="/" component={TournamentListView} />
      <Route path="/tournament/:id" component={TournamentContextProvider}>
        <Route path="/" component={TournamentView}/>
      </Route>
		</Router>
	),
	root!,
);
