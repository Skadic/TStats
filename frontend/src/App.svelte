<script lang="ts">
  import Header from './components/Header.svelte'
  import TournamentView from './components/TournamentView.svelte'
  import Layout from './layouts/Layout.svelte'
</script>


<Layout header headerHeight={64}>

<Header slot="header" text="This is the Header"/>

<TournamentView slot="default"/>

</Layout>