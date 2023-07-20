<script lang="ts">
	export let data;
	let stages = data.stage_id;
</script>

<p>{JSON.stringify(stages)}</p>

<div class="container">
	cool stage dude
</div>

<style>
	.container {
		@apply flex flex-wrap justify-center;
	}

    .container > * {
        @apply flex-1;
    }
</style>
