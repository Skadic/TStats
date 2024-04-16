<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		DeliverAuthCodeRequest,
		DeliverAuthCodeResponse
	} from '$lib/api/osuauth';
	import { tstatsAuthToken, tstatsClient } from '$lib/rpc';

	let csrfToken: string | null = $page.url.searchParams.get('state');
	let authCode: string | null = $page.url.searchParams.get('code');

	async function deliver() {
		const client: OsuAuthServiceClient = tstatsClient(OsuAuthServiceDefinition);
		const request: DeliverAuthCodeRequest = {
			authCode: authCode!,
			state: csrfToken!
		};
		let resp: DeliverAuthCodeResponse = await client.deliverAuthCode(request);
		tstatsAuthToken.set(resp.accessToken);
		goto("/")
	}
</script>

<div>
	{#await deliver()}
		Authenticating...
	{:then}
		Successfully authenticated!
	{:catch error}
		<p style="color: red">Error authenticating: {error.message}</p>
	{/await}
</div>

<style>
</style>
