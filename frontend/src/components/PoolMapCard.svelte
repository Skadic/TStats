<script lang="ts">
	import type { Beatmap } from '$lib/Pool';

	export let map: Beatmap;
	console.log(map)

	function cover() {
		return `url('https://assets.ppy.sh/beatmaps/${map.setId}/covers/cover.jpg')`;
	}
</script>

<div
	class="container rounded-xl p-2 shadow-md shadow-bg-100 transition-all duration-200 hover:scale-105"
	style:--bgstyle={cover()}
>
	<h1 class="text-xl font-bold truncate">{map.name}</h1>
	<div class="flex items-center justify-between">
		<div class="text-lg font-semibold truncate w-1/3">{map.artistName}</div>
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

<style>
	.container {
		background: linear-gradient(to right, rgba(0, 0, 0, 0.5), theme('colors.bg.200')),
			var(--bgstyle);
	}
</style>
