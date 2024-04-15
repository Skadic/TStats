<script lang="ts">
	import { type OsuUserServiceClient, OsuUserServiceDefinition, User } from '$lib/api/osu';
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		RequestAuthCodeResponse
	} from '$lib/api/osuauth';
	import { tstatsClient } from '$lib/rpc';

	async function requestAccess() {
		const client: OsuAuthServiceClient = tstatsClient(OsuAuthServiceDefinition);
		console.log('created client');
		const authCode: RequestAuthCodeResponse = await client.requestAuthCode({});
		console.log(authCode);

		// Navigate to the osu oauth login page
		window.location.href = authCode.authUrl;
	}

	async function getSessionUser() {
		const client: OsuUserServiceClient = tstatsClient(OsuUserServiceDefinition);
		const user: User = await client.get({});
		return {
			username: user.username,
			avaUrl: `https://a.ppy.sh/${user.userId}`
		};
	}
</script>

<nav class="flex justify-between px-10">
	<div>
		<a href="/" class="text-6xl font-bold">TStats</a>
	</div>
	<div class="h-20 aspect-square rounded-xl overflow-hidden">
		{#await getSessionUser() then ava}
			<img src={ava.avaUrl} alt="Avatar of {ava.username}" />
		{:catch}
			<button on:click={requestAccess} class="h-full w-full bg-white"/>
		{/await}
	</div>
</nav>
