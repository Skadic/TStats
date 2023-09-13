import type { PoolBracket } from "./Pool"

export type Stage = {
    tournamentId: number,
    stageOrder: number,
    name: string,
    bestOf: number,
}

export class ExtendedStage {
    tournamentId: string;
    stageOrder: number;
    name: string;
    bestOf: number;
    poolBrackets: PoolBracket[];

    constructor(tournamentId: string, stageOrder: number, name: string, bestOf: number, poolBrackets: PoolBracket[]) {
        this.tournamentId = tournamentId;
        this.stageOrder = stageOrder;
        this.name = name;
        this.bestOf = bestOf;
        this.poolBrackets = poolBrackets;
    }

    addBracket(poolBracket: PoolBracket) {
        this.poolBrackets.push(poolBracket);
    }
}


