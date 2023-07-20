import type { Tournament } from "$lib/Tournament";

export async function load({ fetch, params }) {
    const res = await fetch(`http://127.0.0.1:3000/api/tournament/all`, {
        method: "GET",
        headers: new Headers({
            'Content-Type': "application/json",
            'Access-Control-Allow-Origin': 'http://localhost:3000'
        }),
    });
    const tournaments: Tournament[] = await res.json();

    return { 
        tournaments: tournaments,
    };
}