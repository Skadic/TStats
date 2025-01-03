import { RouteSectionProps } from "@solidjs/router";
import Navbar from "../components/Navbar";
import { ParentComponent } from "../lib/types";

const Layout: ParentComponent<RouteSectionProps<any>> = (props) => {
	return (
		<div class="contents">
			<div class="flex flex-col">
				<Navbar />
				<div class="flex flex-col">{props.children}</div>
			</div>
		</div>
	);
};

export default Layout;
