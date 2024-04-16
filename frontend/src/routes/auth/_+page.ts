import { browser } from '$app/environment';
import {
	DeliverAuthCodeRequest,
	DeliverAuthCodeResponse,
	OsuAuthServiceClient,
	OsuAuthServiceDefinition
} from '$lib/api/osuauth';
import { tstatsAuthToken, tstatsClient } from '$lib/rpc';
import { redirect } from '@sveltejs/kit';

export async function load({ params, url }) {
	// let csrfToken: string | null = url.searchParams.get('state');
	// let authCode: string | null = url.searchParams.get('code');

	// if (browser) {
	// 	const client: OsuAuthServiceClient = tstatsClient(OsuAuthServiceDefinition);
	// 	const request: DeliverAuthCodeRequest = {
	// 		authCode: authCode!,
	// 		state: csrfToken!
	// 	};
	// 	let resp: DeliverAuthCodeResponse = await client.deliverAuthCode(request);
	// 	tstatsAuthToken.set(resp.accessToken);
	// }

	// redirect(308, '/');
}
