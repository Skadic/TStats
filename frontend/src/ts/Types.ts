export interface Tournament {
    id: number;
    shorthand: string,
    full_name: string,
}

export interface Stage {
    id: number,
    tournament: number,
    stage_number: number,
    name: string
}