import createClient, { type Client } from "openapi-fetch";
import type { paths } from "./api/v1";
import { BACKEND_URI } from "./vars";

export function tstatsClient(): Client<paths> {
	return createClient<paths>({ baseUrl: BACKEND_URI });
}
