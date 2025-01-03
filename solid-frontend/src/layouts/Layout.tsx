import { RouteSectionProps } from "@solidjs/router";
import Navbar from "../components/Navbar";
import { ParentComponent } from "../lib/types";
import { AuthContext, fetchSignedInUser } from "../lib/auth";
import { createResource, createSignal } from "solid-js";

const Layout: ParentComponent<RouteSectionProps<any>> = (props) => {
	const [signedInUser] = createResource(async () => {
		return fetchSignedInUser().then((user) => {
			return createSignal<number | null>(user);
		})!;
	});

	return (
		<AuthContext.Provider value={signedInUser()!}>
			<div class="contents">
				<div class="flex flex-col">
					<Navbar />
					<div class="flex flex-col">{props.children}</div>
				</div>
			</div>
		</AuthContext.Provider>
	);
};

export default Layout;
