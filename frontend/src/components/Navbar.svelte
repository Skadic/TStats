<script lang="ts">
	import { type OsuUserServiceClient, OsuUserServiceDefinition, User } from '$lib/api/osu';
	import {
		OsuAuthServiceDefinition,
		type OsuAuthServiceClient,
		RequestAuthCodeResponse
	} from '$lib/api/osuauth';
	import { tstatsAuthToken, tstatsClient } from '$lib/rpc';
	import { createAvatar, melt } from '@melt-ui/svelte';
	import { get } from 'svelte/store';
	import Loader from './Loader.svelte';

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

	let user: User | undefined = undefined;

	let {
		elements: { image, fallback },
		states: { loadingStatus },
		options: { src, delayMs }
	} = createAvatar();

	tstatsAuthToken.subscribe((newToken) => {
		if (newToken === null) {
			user = undefined;
			return;
		}
		loadingStatus.set('loading');
		getSessionUser().then((u) => {
			if (u === undefined) {
				return;
			}
			user = u;
			src.set(`https://a.ppy.sh/${user.userId}`);
			loadingStatus.set('loaded');
		});
	});
</script>

<nav class="flex justify-between bg-bg-400 shadow-bg-400 shadow-md border-bg-600 border-b-2">
	<a href="/" class="text-6xl px-10 font-bold text-center my-auto">TStats</a>
	<div class="p-2">
		<div class="h-20 aspect-square rounded-xl overflow-hidden">
			{#if $loadingStatus === 'loading'}
				<Loader />
			{:else}
				<img use:melt={$image} alt="User Avatar" />
				<button use:melt={$fallback} on:click={requestAccess} class="h-full w-full bg-white" />
			{/if}
		</div>
	</div>
</nav>
