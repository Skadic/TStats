import type { Tournament } from "$lib/Tournament";

export async function load({ fetch, params }) {
    const res = await fetch(`http://tstats.skadic.moe/api/tournament/all`, {
        method: "GET",
        headers: new Headers({
            'Content-Type': "application/json",
        }),
    });
    console.log(res.status)
    const tournaments: Tournament[] = await res.json();

    return { 
        tournaments: tournaments,
    };
}
