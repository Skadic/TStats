
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
