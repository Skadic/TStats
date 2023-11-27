<script lang="ts">
	import { page } from '$app/stores';
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		DeliverAuthCodeRequest
	} from '$lib/api/osuauth';
	import { createChannel, createClient } from 'nice-grpc-web';

	let csrfToken: string | null = $page.url.searchParams.get('state');
	let authCode: string | null = $page.url.searchParams.get('code');

	async function deliver() {
		const channel = createChannel('http://0.0.0.0:9900');
		const client: OsuAuthServiceClient = createClient(OsuAuthServiceDefinition, channel);
		const request: DeliverAuthCodeRequest = {
			authCode: authCode!,
			state: csrfToken!
		};
		await client.deliverAuthCode(request);
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
