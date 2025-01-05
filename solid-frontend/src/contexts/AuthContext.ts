import { createContext, createResource, type Resource } from "solid-js";
import { fetchSignedInUser } from "../lib/auth";
import { tstatsClient } from "../lib/rpc";

const client = tstatsClient();
const [signedInUser, { mutate: _mutate, refetch }] = createResource<
	number | null
>(() => fetchSignedInUser(client));

const refetchUser = async () => {
	await refetch();
	return await fetchSignedInUser(client);
};

export const AuthContext = createContext<AuthContextContent | undefined>();

export type AuthContextContent = {
	signedInUser: Resource<number | null>;
	refetchSignedInUser: () => number | null | Promise<number | null>;
};

export function defaultAuthContextContent(): AuthContextContent {
	return { signedInUser, refetchSignedInUser: refetchUser };
}
