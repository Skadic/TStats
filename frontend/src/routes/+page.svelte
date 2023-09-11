<script lang="ts">
	import ImportantTournaments from '../components/ImportantTournaments.svelte';
	import Navbar from '../components/Navbar.svelte';
	import TournamentList from '../components/TournamentList.svelte';
	export let data;
	console.log(data);
</script>

<div>
    <ImportantTournaments></ImportantTournaments>
	<TournamentList tournaments={data.tournaments} />
</div>

<style>
</style>
