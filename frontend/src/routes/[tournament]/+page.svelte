<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import TournamentView from '$lib/components/TournamentView.svelte';
	import Layout from '$lib/layouts/Layout.svelte';
	import { page } from '$app/stores';
	import { getTournament } from '$lib/ts/Tournament';

	let tournamentId = parseInt($page.params.tournament);
	let tournamentPromise = getTournament(tournamentId);
</script>

<Layout header headerHeight={64}>
	{#await tournamentPromise}
		<Header slot="header" text="Waiting..." />
	{:then tournament}
		<Header slot="header" text={tournament.full_name} />
		<TournamentView {tournament} slot="default" />
	{/await}
</Layout>

<style>
</style>
