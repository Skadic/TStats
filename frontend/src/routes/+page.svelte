
<script lang="ts">
    import TournamentList from "../components/TournamentList.svelte";
    export let data;
    console.log(data);
</script>

<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<TournamentList tournaments={data.tournaments}/>