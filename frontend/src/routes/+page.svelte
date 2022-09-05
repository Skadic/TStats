<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import AllView from '$lib/components/AllView.svelte';
	import Layout from '$lib/layouts/Layout.svelte';
</script>

<Layout header headerHeight={64}>
	<Header slot="header" text="This is the Header" />

	<AllView slot="default" />
</Layout>

<style>
</style>
