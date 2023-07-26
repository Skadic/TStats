import type { PoolBracketExtended } from "./PoolBracket"

export type Stage = {
    tournament_id: number,
    stage_order: number,
    name: string,
    best_of: number,
}

export type ExtendedStage = {
    tournament_id: string,
    stage_order: number,
    name: string,
    best_of: number,
    pool_brackets: PoolBracketExtended[],
}


