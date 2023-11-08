<script lang="ts">
	import type { Beatmap, Difficulty, User } from "$lib/api/osu";

	export let map: Beatmap;
	export let bracketName: string;
	export let bracketOrder: number;
	console.log(map);
	let creator: User = map.creator!;
	let difficulty: Difficulty = map.difficulty!;
	function formatLength(seconds: number): string {
		let min = Math.floor(seconds / 60);
		let sec = seconds % 60;

		return (min < 10 ? '0' : '') + min + ':' + (sec < 10 ? '0' : '') + sec;
	}

	function round1(num: number): number {
		return Math.round(num * 10) / 10 
	}
	function round2(num: number): number {
		return Math.round(num * 100) / 100 
	}

	function cover() {
		return `url('https://assets.ppy.sh/beatmaps/${map.mapsetId}/covers/cover.jpg')`;
	}

	function bracketColor(): string {
		switch (bracketName.toLowerCase()) {
			case 'nm': {
				return '#4285f4';
			}
			case 'hd': {
				return '#f4c542';
			}
			case 'hr': {
				return '#FF5050';
			}
			case 'dt': {
				return '#AA88FF';
			}
			case 'fm': {
				return '#90FF90';
			}
			case 'tb': {
				return '#FFA0FF';
			}
			default: {
				return '#FFFFA0';
			}
		}
	}
</script>

<div
	class="container flex flex-col-reverse lg:flex-row justify-between rounded-xl shadow-md shadow-bg-100 transition-all duration-200 hover:scale-105"
	style:--bgstyle={cover()}
>
	<div
		class="flex bracket-bg text-bg font-bold text-3xl lg:py-5 w-full lg:w-24 rounded-b-xl lg:rounded-br-none lg:rounded-l-xl justify-center items-center"
		style:--bgcolor={bracketColor()}
	>
		{bracketName + bracketOrder}
	</div>
	<div class="w-full p-2">
		<div>
			<a
				class="lg:flex lg:flex-col"
				href="https://osu.ppy.sh/beatmapsets/{map.mapsetId}#osu/{map.mapId}"
			>
				<h1 class="text-xl font-bold">{map.name}</h1>
				<div class="text-lg font-semibold">
					{map.artistName} / {creator.username}
				</div>
			</a>
			<div class="flex flex-col lg:flex-row items-start justify-between">
				<h2 class="font-bold bg-accent-400 text-bg p-1 rounded-md mt-5 lg:my-2">{map.difficultyName}</h2>
				<div class="flex items-center justify-between lg:gap-3 pt-5 lg:pt-0">
					<span><b>★</b> {round2(difficulty.stars)}</span>
					<span><b>Length</b> {formatLength(difficulty.length)}</span>
					<div class="flex items-center gap-2">
						<span><b>CS</b> {round1(difficulty.cs)}</span>
						<span><b>AR</b> {round1(difficulty.ar)}</span>
						<span><b>HP</b> {round1(difficulty.hp)}</span>
						<span><b>OD</b> {round1(difficulty.od)}</span>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		background: linear-gradient(to bottom, rgba(0, 0, 0, 0.5), theme('colors.bg.200')),
			var(--bgstyle);
	}
	@media (min-width: 1024px) {
		.container {
			background: linear-gradient(to right, rgba(0, 0, 0, 0.5), theme('colors.bg.200')),
				var(--bgstyle);
		}
	}
	.bracket-bg {
		background-color: var(--bgcolor);
	}
</style>
