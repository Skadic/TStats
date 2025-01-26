import { type Context, createContext } from "solid-js";
import type { Tournament } from "../lib/api/v1";
import type { Resource } from "solid-js";

export type TournamentContextContent = { tournament: Tournament | null };

export const TournamentContext: Context<
	Resource<Tournament | null> | undefined
> = createContext<Resource<Tournament | null> | undefined>();


