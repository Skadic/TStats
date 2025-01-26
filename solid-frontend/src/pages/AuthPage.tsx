import { Navigate, RouteSectionProps, useSearchParams } from "@solidjs/router";
import { Component } from "../lib/types";
import { createResource, Show, useContext } from "solid-js";
import { deliverAuthCode } from "../lib/auth";
import { AuthContext } from "../contexts/AuthContext";

export const AuthPage: Component<RouteSectionProps<any>> = () => {
	const ctx = useContext(AuthContext);
	if (!ctx) {
		console.error("No Auth Context in Auth Page");
		return <></>;
	}
	const { refetchSignedInUser } = ctx;

	const [params, _setParams] = useSearchParams();
	const [returnUrl] = createResource(async () => {
		const response = await deliverAuthCode(
			params.code?.toString()!,
			params.state?.toString()!,
		);
		await refetchSignedInUser();
		return response?.returnUrl;
	});

	return (
		<div class="p-8 flex justify-around">
			<Show
				when={returnUrl()}
				fallback={<div class="text-4xl font-bold">Authorizing...</div>}
			>
				{(returnUrl) => <Navigate href={returnUrl()} />}
			</Show>
		</div>
	);
};
