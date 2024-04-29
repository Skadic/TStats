<script lang="ts">
	import TournamentInfo from '$lib/components/TournamentInfo.svelte';

	export let data;
	let { tournament, rankRanges, countryRestrictions } = data;
</script>

<div class="bg-bg rounded-xl flex flex-col justify-center gap-8">
	<img
		src="https://i.ppy.sh/c654ce3b0a9aa87b1da2526a46141cf723c47935/68747470733a2f2f6f73752e7070792e73682f77696b692f696d616765732f546f75726e616d656e74732f4f57432f323032332f696d672f6f7763323032332d62616e6e65722e6a7067"
		class="banner w-full h-2/3 opacity-60 img-grad object-cover"
		alt="banner for tournament '{tournament.name}'"
	/>

	<div class="lg:w-3/5 m-auto z-10">
		<TournamentInfo {tournament} {rankRanges} {countryRestrictions} />

		<hr class="py-5" />
		<slot />
	</div>
</div>

<style>
	.img-grad {
		mask-image: linear-gradient(to bottom, rgba(0, 0, 0, 1), rgba(0, 0, 0, 0));
		mask-mode: auto;
		margin-bottom: -15%;
	}
</style>