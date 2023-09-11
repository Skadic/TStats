<script>
	import Navbar from '../components/Navbar.svelte';
	import '../styles/app.css';
</script>

<body>
    <Navbar></Navbar>
	<slot />
</body>

<style>
    body {
        margin: 30px;
    }
</style>
