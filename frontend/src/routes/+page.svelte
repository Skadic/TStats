<script lang="ts">
	import ImportantTournaments from '../components/ImportantTournaments.svelte';
	import TournamentList from '../components/TournamentList.svelte';
	export let data;
	console.log(data);
</script>

<div>
    <ImportantTournaments></ImportantTournaments>
	<div class="pb-4"></div>
	<hr>
	<div class="pb-4"></div>
	<TournamentList tournaments={data.tournaments} />
</div>

<style>
</style>
