import { ParentComponent } from "../lib/types";
import { AuthContext, defaultAuthContextContent } from "./AuthContext";

export const AuthContextProvider: ParentComponent<object> = (props) => {
	return (
		<AuthContext.Provider value={defaultAuthContextContent()}>
			{props.children}
		</AuthContext.Provider>
	);
};
