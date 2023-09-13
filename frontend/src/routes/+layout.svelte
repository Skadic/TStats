<script>
	import Navbar from '../components/Navbar.svelte';
	import '../styles/app.css';
</script>

<body>
	<div class="w-full py-4 bg-bg-400 shadow-bg-400 shadow-md">
		<Navbar />
	</div>
	<div class="w-full max-w-full flex flex-col items-center justify-center p-6">
		<div class="max-w-7xl">
			<slot />
		</div>
	</div>
</body>
