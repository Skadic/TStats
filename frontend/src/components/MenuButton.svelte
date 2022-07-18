<script lang="ts">
    import "../styles/global.css"
    import Icon from "svelte-icons-pack/Icon.svelte"
    import FiMenu from "svelte-icons-pack/fi/FiMenu"

    async function click() {
        let json_string = JSON.stringify({
            shorthand: "AGT",
            full_name: "A Good Tournament" 
        });

        let response = await fetch("http://127.0.0.1:8000/api/tournament/create", {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            headers: new Headers({
                'Content-Type': "application/json"
            }),
            body: json_string
        });

        console.log(response)
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