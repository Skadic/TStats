
export type PoolBracket = {
    tournament_id: number,
    stage_order: number,
    bracket_order: number
    name: string,
}

export type PoolBracketExtended = {
    tournament_id: number,
    stage_order: number,
    bracket_order: number
    name: string,
    maps: any[],
}