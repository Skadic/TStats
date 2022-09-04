<script lang="ts">
import "../styles/global.css"
import Header from "../components/Header.svelte";

export let title: string;
</script>

<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="UTF-8" />
		<meta name="viewport" content="width=device-width" />
		<link rel="icon" type="image/x-icon" href="/favicon.ico" />
		<title>{title}</title>
	</head>
	<body>
        <Header text="NM1 of DM7"></Header>
		<main class="w-full pt-12 m-6 grid grid-cols-2">
			<slot />
		</main>
	</body>
</html>

<style>
body { 
    @apply bg-bglight text-tx;
}	
</style>
