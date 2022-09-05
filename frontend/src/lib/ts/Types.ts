export type Tournament = {
    id: number;
    shorthand: string,
    full_name: string,
}

export type Stage = {
    id: number,
    tournament: number,
    stage_number: number,
    name: string
}