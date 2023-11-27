<script lang="ts">
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		RequestAuthCodeResponse
	} from '$lib/api/osuauth';
	import { createChannel, createClient } from 'nice-grpc-web';

	async function requestAccess() {
		const channel = createChannel('http://0.0.0.0:9900');
		const client: OsuAuthServiceClient = createClient(OsuAuthServiceDefinition, channel);
		console.log("created client")
		const authCode: RequestAuthCodeResponse = await client.requestAuthCode({});
		console.log(authCode);

		// Navigate to the osu oauth login page
		window.location.href = authCode.authUrl;
	}
</script>

<nav class="flex justify-between px-10">
	<div>
		<a href="/" class="text-6xl font-bold">TStats</a>
	</div>
	<button on:click={requestAccess}>
		<div class="w-14 h-14 bg-white" />
	</button>
</nav>
