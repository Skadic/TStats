import type { Stage } from './Stage';

export class Tournament {
	id: number;
	name: string;
	shorthand: string;
	format: TournamentFormat;
	num_players: number;
	rank_range: RankRange[];
	bws: boolean;

	constructor(
		name: string,
		shorthand: string,
		format: TournamentFormat,
		num_players: number,
		rank_ranges: RankRange[],
		bws: boolean
	) {
		this.id = 0;
		this.name = name;
		this.shorthand = shorthand;
		this.format = format;
		this.num_players = num_players;
		this.rank_range = rank_ranges;
		this.bws = bws;
	}

	static deserialize(obj: any): Tournament {
		let rr: RankRange[] = [];
		if (obj.rank_range['Tiered'] !== undefined) {
			rr = obj.rank_range['Tiered'].map((rr: any) => new RankRange(rr.min, rr.max));
		} else if (obj.rank_range['Single'] !== undefined) {
			rr = [new RankRange(obj.rank_range['Single'].min, obj.rank_range['Single'].max)];
		}
		let format =
			obj.format.Versus !== undefined ? TournamentFormat.Versus : TournamentFormat.BattleRoyale;
		let num_players = obj.format.Versus !== undefined ? obj.format.Versus : obj.format.BattleRoyale;
		let tm = new Tournament(obj.name, obj.shorthand, format, num_players, rr, obj.bws);
		if (obj.id !== undefined) {
			tm.id = obj.id;
		}
		return tm;
	}

	formatRankRange(): string {
		console.log(this);
		switch (this.rank_range.length) {
			case 0:
				return 'Open Rank';
			case 1:
				return this.rank_range[0].min + '-' + this.rank_range[0].max;
			default:
				return 'Tiered';
		}
	}

	formatTournamentFormat(): string {
		switch (this.format) {
			case TournamentFormat.Versus:
				return this.num_players + 'v' + this.num_players;
			case TournamentFormat.BattleRoyale:
				return this.num_players + ' player Battle Royale';
		}
	}
}

export class ExtendedTournament {
	tournament: Tournament;
	stages: Stage[];
	countryRestrictions: string[];

	constructor(tournament: Tournament, stages: Stage[], country_restrictions: string[]) {
		this.tournament = tournament;
		this.stages = stages;
		this.countryRestrictions = country_restrictions;
	}

	static deserialize(obj: any): ExtendedTournament {
		const tn: Tournament = Tournament.deserialize(obj);
		const stages: Stage[] = obj.stages;
		const countryRestrictions: string[] = obj.country_restrictions;

		let extTn = new ExtendedTournament(tn, stages, countryRestrictions);
		return extTn;
	}
}

export class RankRange {
	min: number;
	max: number;

	constructor(min: number, max: number) {
		this.min = min;
		this.max = max;
	}
}

export enum TournamentFormat {
	Versus = 'Versus',
	BattleRoyale = 'BattleRoyale'
}
