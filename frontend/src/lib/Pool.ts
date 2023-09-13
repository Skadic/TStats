export class PoolMap {
    artistName: string;
    name: string;
    setId: number;
    mapId: number;
    difficulty: Difficulty;

    constructor(artistName: string, name: string, setId: number, mapId: number, difficulty: Difficulty) {
        this.artistName = artistName;
        this.name = name;
        this.setId = setId;
        this.mapId = mapId;
        this.difficulty = difficulty;
    }

    formatLength(): string {
        const min = Math.floor(this.difficulty.length / 60);
        const sec = this.difficulty.length % 60;
        return  min + ":" + (sec < 10 ? "0" + sec : sec);
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

    constructor(stars: number, length: number, bpm: number, cs: number, ar: number, od: number, hp: number) {
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
    maps: PoolMap[];

    constructor(name: string, maps: PoolMap[]) {
        this.name = name;
        this.maps = maps;
    }
}