import { Tournament } from "$lib/Tournament";

export async function load({ fetch, params }) {
    const res = await fetch(`http://0.0.0.0:3000/api/tournament/all`, {
        method: "GET",
        headers: new Headers({
            'Content-Type': "application/json",
        }),
    });
    console.log(res.status)
    const ts: any[] = await res.json();
    console.log(ts);

    return { 
        tournaments: ts.map((t: any) => Tournament.deserialize(t)),
    };
}
