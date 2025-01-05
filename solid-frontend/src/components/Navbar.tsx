import { Image } from "@kobalte/core/image";
import { Component } from "../lib/types";
import { createResource, useContext } from "solid-js";
import { Button } from "@kobalte/core/button";
import { requestAccess } from "../lib/auth";
import { tstatsClient } from "../lib/rpc";
import { A, useLocation } from "@solidjs/router";
import { AuthContext } from "../contexts/AuthContext";

function userAvatar(userId: number | null) {
	return userId ? `https://a.ppy.sh/${userId}` : null;
}

const Navbar: Component = () => {
	const client = tstatsClient();

	const ctx = useContext(AuthContext);
	if (!ctx) {
		console.error("No Auth Context in Navbar");
		return <></>;
	}

	const [signedInUserAvatar] = createResource(ctx.signedInUser, (id) =>
		userAvatar(id),
	);

	const location = useLocation();

	async function redirectToOsuAuthPage() {
		const osuAuthUrl = await requestAccess(location.pathname, client);
		window.location.href = osuAuthUrl!;
	}

	return (
		<nav class="flex justify-between bg-bg-400 shadow-bg-400 shadow-md border-bg-600 border-b-2">
			<A href="/" class="text-6xl px-10 font-bold text-center my-auto">
				TStats
			</A>
			<div class="p-2">
				<div class="h-20 bg-text-500 aspect-square rounded-xl overflow-hidden">
					<Image>
						<Image.Img src={signedInUserAvatar() ?? ""} />
						<Image.Fallback class="bg-white" />
					</Image>
					<Button
						onclick={redirectToOsuAuthPage}
						class="h-full w-full"
						aria-label="Authorize with osu account"
					/>
				</div>
			</div>
		</nav>
	);
};

export default Navbar;
