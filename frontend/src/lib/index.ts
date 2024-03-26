// place files you want to import through the `$lib` alias in this folder.

import { PUBLIC_BACKEND_HOST, PUBLIC_BACKEND_METHOD, PUBLIC_BACKEND_PORT } from "$env/static/public";

export const BACKEND_URI: string = PUBLIC_BACKEND_METHOD + '://' + PUBLIC_BACKEND_HOST + ':' + PUBLIC_BACKEND_PORT;
