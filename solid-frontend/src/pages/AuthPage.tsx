import { Navigate, RouteSectionProps, useSearchParams } from "@solidjs/router";
import { Component } from "../lib/types";
import { createResource, Match, Switch, useContext } from "solid-js";
import { AuthContext, deliverAuthCode } from "../lib/auth";

const AuthPage: Component<RouteSectionProps<any>> = () => {
	const [signedInUser, setSignedInUser] = useContext(AuthContext);

	const [params, _] = useSearchParams();
	const [returnUrl] = createResource(async () => {
		const response = await deliverAuthCode(
			params.code?.toString()!,
			params.state?.toString()!,
		);

		setSignedInUser(response?.userId ?? null);
		return response?.returnUrl;
	});

	return (
		<div class="p-8 flex justify-around">
			<Switch fallback={<div class="text-4xl font-bold">Authorizing...</div>}>
				<Match when={returnUrl()}>
					<Navigate href={returnUrl()!} />
				</Match>
			</Switch>
		</div>
	);
};

export default AuthPage;
