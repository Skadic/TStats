import type { Tournament } from "./Types";

export async function addTournament(shorthand: string, full_name: string) {
    let json_string = JSON.stringify({
        shorthand: shorthand,
        full_name: full_name 
    });

    let response = await fetch("http://127.0.0.1:8000/api/tournament/create", {
        method: "POST",
        mode: "cors",
        cache: "no-cache",
        headers: new Headers({
            'Content-Type': "application/json"
        }),
        body: json_string
    });

    console.log(response)
}

export async function getAllTournaments(): Promise<Tournament[]> {
    const res = await fetch(`http://127.0.0.1:8000/api/tournament/all`, {
        method: "GET",
        mode: "cors",
        cache: "no-cache",
        headers: new Headers({
            'Content-Type': "application/json"
        }),
    });
    const json = await res.json();
    return json;
}

export async function getTournament(tournament: number): Promise<Tournament> {
    const res = await fetch(`http://127.0.0.1:8000/api/tournament/${tournament}`, {
        method: "GET",
        mode: "cors",
        cache: "no-cache",
        headers: new Headers({
            'Content-Type': "application/json"
        }),
    });
    const json = await res.json();
    return json;
}