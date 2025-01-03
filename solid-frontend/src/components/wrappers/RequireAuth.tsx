import { createAsync, RouteSectionProps, useLocation } from "@solidjs/router";
import { Component, ParentComponent } from "../../lib/types";
import { Match, Suspense, Switch } from "solid-js";
import { fetchSignedInUser } from "../../lib/auth";
import Authorize from "../Authorize";

const RequireAuth: ParentComponent<
	RouteSectionProps<{ component: Component<any> }>
> = (props) => {
	const isAuthenticated = createAsync(() =>
		fetchSignedInUser().then((userId) => {
			return userId !== null;
		}),
	);

	const location = useLocation();

	return (
		<Suspense fallback={<div>Checking Auth...</div>}>
			<Switch fallback={<Authorize returnUrl={location.pathname} />}>
				<Match when={isAuthenticated()}>{props.data.component}</Match>
			</Switch>
		</Suspense>
	);
};

export default RequireAuth;
