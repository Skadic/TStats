<script lang="ts">
	import type { Beatmap } from '$lib/Pool';

	export let map: Beatmap;
	export let bracketName: string;
	export let bracketOrder: number;
	console.log(map);

	function cover() {
		return `url('https://assets.ppy.sh/beatmaps/${map.setId}/covers/cover.jpg')`;
	}
</script>

<div
	class="flex items-center justify-between container rounded-xl shadow-md shadow-bg-100 transition-all duration-200 hover:scale-105"
	style:--bgstyle={cover()}
>
	<div class="bg-secondary text-bg w-1/10 font-bold text-3xl max-h-full h-full py-5 rounded-l-xl w-24 flex justify-center">{bracketName + bracketOrder}</div>
	<div class="w-full p-2">
		<a
			class="flex justify-between items-center"
			href="https://osu.ppy.sh/beatmapsets/{map.setId}#osu/{map.mapId}"
		>
			<h1 class="text-xl font-bold truncate">{map.name}</h1>
			<h2 class="text-lg font-bold truncate">[{map.diffName}]</h2>
		</a>
		<div class="flex flex-col lg:flex-row items-center justify-between">
			<div class="text-lg font-semibold truncate w-2/5">
				{map.artistName} / {map.creator.username}
			</div>
			<div class="flex items-center justify-between gap-4 max-w-fit">
				<span><b>★</b> {map.difficulty.stars}</span>
				<span><b>Length</b> {map.formatLength()}</span>
				<div class="flex items-center gap-2">
					<span><b>CS</b> {map.difficulty.cs}</span>
					<span><b>AR</b> {map.difficulty.ar}</span>
					<span><b>HP</b> {map.difficulty.hp}</span>
					<span><b>OD</b> {map.difficulty.od}</span>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.container {
		background: linear-gradient(to right, rgba(0, 0, 0, 0.5), theme('colors.bg.200')),
			var(--bgstyle);
	}
</style>
