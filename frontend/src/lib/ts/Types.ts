export type Tournament = {
    id: number,
    shorthand: string,
    full_name: string,
    play_format: number,
    team_size: number,
    score_version: number,
}

export type Stage = {
    id: number,
    tournament_id: number,
    idx: number,
    stage_name: string,
    best_of: number
}