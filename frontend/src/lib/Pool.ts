import type { OsuUser } from './User';

export class Beatmap {
	artistName: string;
	name: string;
	diffName: string;
	setId: number;
	mapId: number;
	creator: OsuUser;
	difficulty: Difficulty;

	constructor(
		artistName: string,
		name: string,
		diffName: string,
		setId: number,
		mapId: number,
		creator: OsuUser,
		difficulty: Difficulty
	) {
		this.artistName = artistName;
		this.name = name;
		this.diffName = diffName;
		this.setId = setId;
		this.mapId = mapId;
		this.creator = creator;
		this.difficulty = difficulty;
	}

	formatLength(): string {
		const min = Math.floor(this.difficulty.length / 60);
		const sec = this.difficulty.length % 60;
		return min + ':' + (sec < 10 ? '0' + sec : sec);
	}

	static deserialize(obj: any): Beatmap {
		return new Beatmap(
			obj.artistName,
			obj.name,
			obj.diffName,
			obj.setId,
			obj.mapId,
			obj.creator,
			obj.difficulty
		);
	}
}

export class Difficulty {
	stars: number;
	length: number;
	bpm: number;
	cs: number;
	ar: number;
	od: number;
	hp: number;

	constructor(
		stars: number,
		length: number,
		bpm: number,
		cs: number,
		ar: number,
		od: number,
		hp: number
	) {
		this.stars = stars;
		this.length = length;
		this.cs = cs;
		this.bpm = bpm;
		this.ar = ar;
		this.od = od;
		this.hp = hp;
	}
}

export class PoolBracket {
	name: string;
	maps: Beatmap[];

	constructor(name: string, maps: Beatmap[]) {
		this.name = name;
		this.maps = maps;
	}

	static deserialize(obj: any): PoolBracket {
        let arr: Beatmap[] = []

        for (const map of obj.maps) {
            arr.push(Beatmap.deserialize(map))
        }
        return new PoolBracket(obj.name, arr)
    }
}
