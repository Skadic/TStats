<script>
	import Navbar from '../components/Navbar.svelte';
	import '../styles/app.css';
</script>

<body class="m-2 lg:m-4">
    <Navbar></Navbar>
	<slot />
</body>

<style>
	body {
        margin: 0 auto;
        max-width: 1200px;
	}
</style>
