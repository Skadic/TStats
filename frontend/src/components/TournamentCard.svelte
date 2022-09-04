<script lang="ts">
import "../styles/app.css"
import { getAllStages } from "../ts/Stage";
import type { Tournament } from "../ts/Types";


export let tournament: Tournament;

async function openTournament() {
    let stages = await getAllStages(tournament.id);
    console.log(stages)
}
</script>

<div class="tcard">
    <a href="Pool" class="block w-full h-full">{tournament.full_name} ({tournament.shorthand})</a>
</div>

<style>

.tcard {
    @apply bg-primary w-full h-16 rounded-xl flex font-nunito text-tx p-1 pl-2 mb-2 mr-2;
}

</style>