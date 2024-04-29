<script lang="ts">
	import type { User } from '$lib/api/osu';
	import type { Score } from '$lib/api/scores';
	import { createAvatar } from '@melt-ui/svelte';
	import Flag from './Flag.svelte';

	export let score: Score;
	export let rank: number;

	let user: User = score.user!;
	let points: number = score.score;

	let {
		elements: { image, fallback },
		states: { loadingStatus },
		options: { src, delayMs }
	} = createAvatar({ src: `https://a.ppy.sh/${user.userId}` });

	function cover() {
		let url =
			user.coverUrl.lastIndexOf('.') > 3
				? `https://assets.ppy.sh/user-profile-covers/${user.userId}/${user.coverUrl}`
				: `https://osu.ppy.sh/images/headers/profile-covers/${user.coverUrl}`;

		return `url('${url}')`;
	}
	function numColor(): string {
		switch (rank) {
			case 1:
				return 'text-gold';
			case 2:
				return 'text-silver';
			case 3:
				return 'text-bronze';
			default:
				return '';
		}
	}

	function commatize(n: number): string {
		return n.toLocaleString("en-us", { notation: 'standard' });
	}
</script>

<div class="flex h-16">
	<div class="grid place-items-center enter aspect-square">
		<div class="text-5xl font-bold {numColor()}">{rank}</div>
	</div>

	<div class="flex rounded-2xl overflow-hidden w-full shadow-bg shadow-xl">
		<div class="h-full aspect-square">
			<img use:melt={$image} class="h-full" alt="{user.username}'s Avatar" />
		</div>
		<div
			class="px-4 actually-align-center flex align-center justify-between w-full banner-bg"
			style:--bgstyle={cover()}
		>
			<div class="actually-align-center flex justify-center align-center gap-4 h-full">
				<span class="h-min text-4xl font-bold">{user.username}</span>
				<span class="h-min text-3xl">
					<Flag extraStyles="rounded-md" country={user.country} />
				</span>
			</div>
			<div class="text-4xl font-semibold">{commatize(score.score)}</div>
		</div>
	</div>
</div>

<style>
	.banner-bg {
		background: linear-gradient(90deg, rgba(50, 50, 50, 0.75) 0%, theme('colors.bg.200') 75%),
			var(--bgstyle);
		background-size: 300px;
		background-repeat: no-repeat;
		background-position: 0%;
		background-size: cover;
	}

	.actually-align-center > * {
		margin-top: auto;
		margin-bottom: auto;
	}
</style>
