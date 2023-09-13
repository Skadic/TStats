export type BracketResult = {
	name: string,
	maps: number[]
}

export type StageResult = {
	tournamentId: number,
	name: string,
	stageOrder: number,
	bestOf: number,
	brackets: BracketResult[]
}

export async function load({ fetch, params }) {
	return {
        stageId: params.stage,
	};
}
