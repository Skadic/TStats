<script lang="ts">
	import ImportantTournaments from '$lib/components/ImportantTournaments.svelte';
	import TournamentList from '$lib/components/TournamentList.svelte';
	export let data;
	console.log(data);
</script>

<div class="lg:w-3/5 m-auto">
	<div class="py-2"></div>
	<div class="px-6 py-4">
		<ImportantTournaments />
	</div>
	<hr class="py-4" />
	<div class="px-1">
		<TournamentList tournaments={data.tournaments} />
	</div>
</div>

<style>
</style>
