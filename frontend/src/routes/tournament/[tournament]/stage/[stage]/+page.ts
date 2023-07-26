export type BracketResult = {
	name: string,
	maps: number[]
}

export type StageResult = {
	tournament_id: number,
	name: string,
	stage_order: number,
	best_of: number,
	brackets: BracketResult[]
}

export async function load({ fetch, params }) {
	return {
        stage_id: params.stage,
	};
}
