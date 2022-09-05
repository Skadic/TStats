<script lang="ts">
	let main: HTMLElement;

	export let header = false;
	export let headerHeight = 64;

	export let footer = false;
	export let footerHeight = 64;

	console.log('Yes this is doing something');

	$: if (main) {
		main.style.setProperty('--header-height', headerHeight + 'px');
		main.style.setProperty('--footer-height', footerHeight + 'px');
	}
</script>

<main bind:this={main} class:header class:footer>
	<slot />
</main>

<div class="slotHeader">
	<slot name="header" />
</div>

<div class="slotFooter">
	<slot name="footer" />
</div>

<style>
	main {
		@apply h-full w-full bg-bglight;
	}

	main.header {
		padding-top: var(--header-height, 0px);
	}

	main.footer {
		padding-bottom: var(--footer-height, 0px);
	}

	.slotHeader {
		@apply fixed top-0 left-0 w-full;
	}

	.slotFooter {
		@apply fixed bottom-0 left-0 w-full;
	}
</style>
