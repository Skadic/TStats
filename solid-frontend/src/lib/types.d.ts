import type { RouteSectionProps } from "@solidjs/router";
import type { JSX } from "solid-js";

export type EmptyObject = Record<string, never>;
export type Component<P = EmptyObject> = (props: P) => JSX.Element;
export type PageComponent<P = EmptyObject> = Component<RouteSectionProps<P>>;
export type ParentProps<P = EmptyObject> = P & {
	children?: JSX.Element;
};
export type ParentComponent<P = EmptyObject> = Component<ParentProps<P>>;
export type LayoutComponent<P = EmptyObject> = ParentComponent<RouteSectionProps<P>>;
