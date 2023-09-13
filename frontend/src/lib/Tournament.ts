import type { Stage } from './Stage';

export class Tournament {
	id: number;
	name: string;
	shorthand: string;
	format: TournamentFormat;
	numPlayers: number;
	rankRange: RankRange[];
	bws: boolean;

	constructor(
		name: string,
		shorthand: string,
		format: TournamentFormat,
		numPlayers: number,
		rankRanges: RankRange[],
		bws: boolean
	) {
		this.id = 0;
		this.name = name;
		this.shorthand = shorthand;
		this.format = format;
		this.numPlayers = numPlayers;
		this.rankRange = rankRanges;
		this.bws = bws;
	}

	static deserialize(obj: any): Tournament {
		let rr: RankRange[] = [];
		if (obj.rankRange['Tiered'] !== undefined) {
			rr = obj.rankRange['Tiered'].map((rr: any) => new RankRange(rr.min, rr.max));
		} else if (obj.rankRange['Single'] !== undefined) {
			rr = [new RankRange(obj.rankRange['Single'].min, obj.rankRange['Single'].max)];
		}
		let format =
			obj.format.Versus !== undefined ? TournamentFormat.Versus : TournamentFormat.BattleRoyale;
		let numPlayers = obj.format.Versus !== undefined ? obj.format.Versus : obj.format.BattleRoyale;
		let tm = new Tournament(obj.name, obj.shorthand, format, numPlayers, rr, obj.bws);
		if (obj.id !== undefined) {
			tm.id = obj.id;
		}
		return tm;
	}

	formatRankRange(): string {
		console.log(this);
		switch (this.rankRange.length) {
			case 0:
				return 'Open Rank';
			case 1:
				return this.rankRange[0].min + '-' + this.rankRange[0].max;
			default:
				return 'Tiered';
		}
	}

	formatTournamentFormat(): string {
		switch (this.format) {
			case TournamentFormat.Versus:
				return this.numPlayers + 'v' + this.numPlayers;
			case TournamentFormat.BattleRoyale:
				return this.numPlayers + ' player Battle Royale';
		}
	}
}

export class ExtendedTournament {
	tournament: Tournament;
	stages: Stage[];
	countryRestrictions: string[];

	constructor(tournament: Tournament, stages: Stage[], countryRestrictions: string[]) {
		this.tournament = tournament;
		this.stages = stages;
		this.countryRestrictions = countryRestrictions;
	}

	static deserialize(obj: any): ExtendedTournament {
		const tn: Tournament = Tournament.deserialize(obj);
		const stages: Stage[] = obj.stages;
		const countryRestrictions: string[] = obj.countryRestrictions;

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
