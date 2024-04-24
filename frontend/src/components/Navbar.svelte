<script lang="ts">
	import { type OsuUserServiceClient, OsuUserServiceDefinition, User } from '$lib/api/osu';
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		RequestAuthCodeResponse
	} from '$lib/api/osuauth';
	import { tstatsAuthToken, tstatsClient } from '$lib/rpc';
	import { get } from 'svelte/store';

	async function requestAccess() {
		const client: OsuAuthServiceClient = tstatsClient(OsuAuthServiceDefinition);
		const authCode: RequestAuthCodeResponse = await client.requestAuthCode({});

		// Navigate to the osu oauth login page
		window.location.href = authCode.authUrl;
	}

	async function getSessionUser() {
		if (get(tstatsAuthToken) === null) {
			return undefined;
		}
		try {
			const client: OsuUserServiceClient = tstatsClient(OsuUserServiceDefinition);
			const user: User | undefined = (await client.get({})).user;
			if (user !== undefined) {
				return user;
			} else return undefined;
		} catch {
			return undefined;
		}
	}

	let user: any = undefined;

	tstatsAuthToken.subscribe((_) => {
		getSessionUser().then((u) => (user = u));
	});
</script>

<nav class="flex justify-between px-10">
	<div>
		<a href="/" class="text-6xl font-bold">TStats</a>
	</div>
	<div class="h-20 aspect-square rounded-xl overflow-hidden">
		{#if user !== undefined}
			<img src={`https://a.ppy.sh/${user.userId}`} alt="Avatar of {user.username}" />
		{:else}
			<button on:click={requestAccess} class="h-full w-full bg-white" />
		{/if}
	</div>
</nav>
