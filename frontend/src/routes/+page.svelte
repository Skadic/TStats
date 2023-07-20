
<script lang="ts">
    import TournamentList from "../components/TournamentList.svelte";
    export let data;
    console.log(data);
</script>

<TournamentList tournaments={data.tournaments}/>