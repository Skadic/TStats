import { createResource, Match, Suspense, Switch } from "solid-js";
import { Component } from "../lib/types";
import { requestAccess } from "../lib/auth";
import { Navigate } from "@solidjs/router";

const Authorize: Component<{ returnUrl: string }> = (props) => {
	const [osuOauthUrl] = createResource(() => requestAccess(props.returnUrl));
	return <Navigate href={osuOauthUrl()!} />;
};

export default Authorize;
