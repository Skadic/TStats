import type { Stage } from "./Types";

export async function getAllStages(tournament: number): Promise<Stage[]> {
    return fetch(`http://127.0.0.1:8000/api/tournament/${tournament}/stage`, {
        method: "GET",
        mode: "cors",
        cache: "no-cache",
        headers: new Headers({
            'Content-Type': "application/json"
        }),
    })
    .then(response => response.json());
}