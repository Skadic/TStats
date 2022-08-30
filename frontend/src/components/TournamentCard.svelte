<script lang="ts">
import "../styles/global.css"
import { getAllStages } from "../ts/Stage";
import { Tournament } from "../ts/Types";

export let tournament: Tournament;

async function openTournament() {
    let stages = await getAllStages(tournament.id);
    console.log(stages)
}

</script>

<button class="tcard" on:click="{openTournament}">
    {tournament.full_name} ({tournament.shorthand})
</button>

<style>

.tcard {
    @apply bg-primary w-auto h-auto rounded-xl flex items-center font-nunito text-tx p-1 pl-2 mb-2 mr-2;
}

</style>