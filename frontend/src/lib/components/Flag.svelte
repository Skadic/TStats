<script lang="ts">
	import '/node_modules/flag-icons/css/flag-icons.min.css';

	export let country: string;
	export let extraStyles: string = '';
</script>

<span class="bg-center flag fi fi-{country.toLowerCase()} {extraStyles}" />

<style>
	.flag {
		aspect-ratio: 4/3;
	}
</style>
