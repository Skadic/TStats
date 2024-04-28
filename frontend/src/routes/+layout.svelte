<script>
	import Navbar from '../components/Navbar.svelte';
	import '../styles/app.css';
</script>

<body class="flex flex-col">
	<Navbar />
	<div class="flex flex-col">
		<slot />
	</div>
</body>
