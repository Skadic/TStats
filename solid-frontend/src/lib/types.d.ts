import type { JSX } from "solid-js";

type EmptyObject = Record<string, never>;
export type Component<P = EmptyObject> = (props: P) => JSX.Element;
export type ParentProps<P = EmptyObject> = P & {
	children?: JSX.Element;
};
export type ParentComponent<P = EmptyObject> = Component<ParentProps<P>>;
