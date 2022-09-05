<script lang="ts">
  import Header from '$lib/components/Header.svelte'
  import TournamentView from '$lib/components/TournamentView.svelte'
  import Layout from '$lib/layouts/Layout.svelte'
</script>


<div class="app">
<Layout header headerHeight={64}>

<Header slot="header" text="This is the Header, but pools!"/>

<TournamentView slot="default"/>

</Layout>
</div>

<style>
.app {
  @apply bg-bglight;
}
</style>