<script lang="ts">
	export let data;
	let stages = data.stageId;
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
