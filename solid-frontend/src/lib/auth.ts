import { type TStatsClient, tstatsClient } from "./rpc";
import type { DeliverAuthCodeResponse } from "./api/v1types";

export async function requestAccess(
	returnUrl: string,
	client: TStatsClient = tstatsClient(),
): Promise<string | undefined> {
	const response = await client.GET("/api/auth", {
		params: { query: { returnUrl: returnUrl } },
	});

	if (response.data?.authUrl !== undefined) {
		return response.data?.authUrl;
	}

	console.error("could not request access code");
	return undefined;
}

export async function deliverAuthCode(
	authCode: string,
	csrfToken: string,
	client: TStatsClient = tstatsClient(),
): Promise<DeliverAuthCodeResponse | undefined> {
	const response = await client.POST("/api/auth", {
		body: {
			authCode,
			state: csrfToken,
		},
	});

	if (response.data !== undefined) {
		return response.data;
	}

	console.error("could not deliver auth code");
	return undefined;
}

export async function fetchSignedInUser(
	client: TStatsClient = tstatsClient(),
): Promise<number | null> {
	return client
		.GET("/api/auth/user", {
			params: {
				cookie: undefined,
			},
		})
		.then((user) => {
			if (user.data?.userId) {
				return user.data.userId;
			}
			console.error(`error fetching signed in user: ${user.error}`);
			return null;
		});
}

export async function fetchSignedInUserAvatar(
	client: TStatsClient = tstatsClient(),
): Promise<string | null> {
	return fetchSignedInUser(client).then((userId) => {
		return userId ? `https://a.ppy.sh/${userId}` : null;
	});
}
