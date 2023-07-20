import type { Stage } from "./Stage";

export type Tournament = {
    id: string,
    name: string,
    shorthand: string,
    format: any,
    rank_range: any,
    bws: boolean,
    country_restriction: string[],
}

export type ExtendedTournament = {
    id: string,
    name: string,
    shorthand: string,
    format: any,
    rank_range: any,
    bws: boolean,
    country_restriction: string[],
    stages: Stage[],
}

export type RankRange = {
    start: number,
    end: number
}

export function formatRankRange(tournament: any): string {
    if (tournament.rank_range === null) {
        return "Open Rank";
    } if (tournament.rank_range["Tiered"] !== undefined) {
        return "Tiered";
    } else {
        return tournament.rank_range["Single"].start + "-" + tournament.rank_range["Single"].end;
    }
}

export function formatRankRangeDetailed(tournament: any): string[] {
    if (tournament.rank_range === null) {
        return ["Open Rank"];
    } if (tournament.rank_range["Tiered"] !== undefined) {
        return tournament.rank_range["Tiered"].map((o: RankRange, i: number) => `Tier ${i+1}: ` + o.start + "-" + o.end);
    } else {
        return [tournament.rank_range["Single"].start + "-" + tournament.rank_range["Single"].end];
    }
}

export function formatTournamentFormat(tournament: any): string {
    if (tournament.format["Versus"] !== undefined){
        let players = tournament.format["Versus"]
        return players + "v" + players;
    } else {
        return tournament.format["BattleRoyale"] + " player Battle Royale";
    }
    return ""
}