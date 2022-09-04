<script lang="ts">
import "../styles/app.css"
import { getAllTournaments } from '../ts/Tournament';
import TournamentCard from './TournamentCard.svelte';

let promise = getAllTournaments();
</script>

<div class="tview-container">
    {#await promise}
    	<p>...waiting</p>
    {:then tournaments}
        {#each tournaments as tournament}
            <TournamentCard tournament={tournament}/>
        {/each}
    {:catch error}
    	<p style="color: red">{error.message}</p>
    {/await}
</div>

<style>
.tview-container {
    @apply w-auto overflow-y-auto m-2 grid grid-cols-2 gap-2;
}
</style>


