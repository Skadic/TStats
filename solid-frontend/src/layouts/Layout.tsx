import Navbar from "../components/Navbar";
import { AuthContextProvider } from "../contexts/AuthContextProvider";
import { LayoutComponent } from "../lib/types";

const Layout: LayoutComponent<any> = (props) => {
	return (
		<div class="contents">
			<div class="flex flex-col">
				<AuthContextProvider>
					<Navbar />
					<div class="flex flex-col">{props.children}</div>
				</AuthContextProvider>
			</div>
		</div>
	);
};

export default Layout;
