<script lang="ts">
	import ImportantTournaments from '../components/ImportantTournaments.svelte';
	import TournamentList from '../components/TournamentList.svelte';
	export let data;
	console.log(data);
</script>

<div class="">
	<div class="px-6">
		<ImportantTournaments />
	</div>
	<div class="py-4" />
	<hr />
	<div class="py-4" />
	<div class="px-1">
		<TournamentList tournaments={data.tournaments} />
	</div>
</div>

<style>
</style>
