<script lang="ts">
	import '$lib/styles/app.css';

	import MenuButton from './MenuButton.svelte';

	export let text: string;
</script>

<nav class="header font-nunito">
	<MenuButton /> <span class="header-text">{text}</span>
</nav>

<style>
	.header {
		@apply bg-bgdark flex top-0 left-0 fixed w-screen h-auto items-center 
        transition-all duration-300 text-3xl;
	}

	.header-text {
		@apply text-tx;
	}

	.header:hover {
		@apply rounded-b-xl;
	}
	.header:hover .header-text {
		@apply font-black text-tx;
	}
</style>
