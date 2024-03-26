<script>
	import Navbar from '../components/Navbar.svelte';
	import '../styles/app.css';
</script>

<body class="flex flex-col gap-10">
	<div class="py-4 bg-bg-400 shadow-bg-400 shadow-md">
		<Navbar />
	</div>
	<div class="lg:w-3/5 m-auto flex flex-col justify-center">
		<slot />
	</div>
</body>
