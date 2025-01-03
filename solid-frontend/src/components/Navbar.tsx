import { Component } from "../lib/types";

const Navbar: Component = () => {
	// {#await signedInUser}
	// 	<Loader />
	// {:then}
	// 	<img use:melt={$image} alt="User Avatar" />
	// 	<button use:melt={$fallback} onclick={requestAccess} class="h-full w-full bg-white" aria-label="Authorize with osu account"></button>
	// {/await}

	return (
		<nav class="flex justify-between bg-bg-400 shadow-bg-400 shadow-md border-bg-600 border-b-2">
			<a href="/" class="text-6xl px-10 font-bold text-center my-auto">
				TStats
			</a>
			<div class="p-2">
				<div class="h-20 aspect-square rounded-xl overflow-hidden"></div>
			</div>
		</nav>
	);
};

export default Navbar;
