export type StageResult  = {
	name: string,
	best_of: number	
}

export type TournamentResult = {
	id: number,
	name: string,
	shorthand: string,
	format: any,
	rank_range: any,
	bws: boolean,
	stages: StageResult[]
	country_restrictions: string[]
}

export async function load({ fetch, params }): Promise<TournamentResult> {
	
	const res = await fetch(
		`http://tstats.skadic.moe/api/tournament?` +
			new URLSearchParams({
				id: params.tournament,
			}),
		{
			method: 'GET',
			headers: new Headers({
				'Content-Type': 'application/json',
			})
		}
	);
	
	const js = await res.json();
	//console.log(js);
	const tournament: TournamentResult = js;

	return tournament;
}
