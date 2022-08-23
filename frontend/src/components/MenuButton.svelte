<script lang="ts">
    import "../styles/global.css"
    import Icon from "svelte-icons-pack/Icon.svelte"
    import FiMenu from "svelte-icons-pack/fi/FiMenu"
    import { addTournament } from "../ts/tournament";

    async function click() {
        addTournament("BTE", "Best Tournament Ever")
    }
</script>

<style>
    .menu-container {
        @apply bg-primary text-bgdark w-10 h-10 rounded-lg m-2 flex justify-center items-center transition-all;
    }

    .menu-container:hover {
        @apply bg-secondary;
    }
</style>


<button class="menu-container" on:click="{click}">
    <Icon src={FiMenu}/>
</button>