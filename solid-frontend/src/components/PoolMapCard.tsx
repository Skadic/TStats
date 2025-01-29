
import { Beatmap, Difficulty, OsuUser, PoolBracket, Stage } from "../lib/api/v1";
import { Component } from "../lib/types";

import styles from "./PoolMapCard.module.css"

function formatLength(seconds: number): string {
    const min = Math.floor(seconds / 60);
    const sec = seconds % 60;

    return (min < 10 ? "0" : "") + min + ":" + (sec < 10 ? "0" : "") + sec;
}

function round1(num: number): number {
    return Math.round(num * 10) / 10;
}
function round2(num: number): number {
    return Math.round(num * 100) / 100;
}

function bracketColor(bracketName: string): string {
    switch (bracketName.toLowerCase()) {
        case "nm": {
            return "#4285f4";
        }
        case "hd": {
            return "#f4c542";
        }
        case "hr": {
            return "#FF5050";
        }
        case "dt": {
            return "#AA88FF";
        }
        case "fm": {
            return "#90FF90";
        }
        case "tb": {
            return "#FFA0FF";
        }
        default: {
            return "#FFFFA0";
        }
    }
}

function coverUrl(mapsetId: number) {
    return `url('https://assets.ppy.sh/beatmaps/${mapsetId}/covers/cover.jpg')`;
}

export const PoolMapCard: Component<{ tournamentId: number, stageOrder: number, bracket: PoolBracket, mapOrder: number, map: Beatmap }> = (props) => {
    const { map, tournamentId, stageOrder, bracket, mapOrder } = props;

    const creator: OsuUser = map.creator!;
    const difficulty: Difficulty = map.difficulty!;

    const link: string | undefined =
        `/tournament/${tournamentId}/stage/${stageOrder}/map/${bracket.order}/${mapOrder}`;

    return <div
        class={`${styles.container} flex flex-col-reverse lg:flex-row justify-between rounded-xl shadow-md shadow-bg-100`}
        style={`--bgstyle: ${coverUrl(map.mapsetId)};`}
    >
        <div
            class={`${styles.bracketBg} flex text-bg font-bold text-3xl lg:py-5 w-full lg:w-24 rounded-b-xl lg:rounded-br-none lg:rounded-l-xl justify-center items-center`}
            style={`--bgcolor: ${bracketColor(bracket.name)};`}
        >
            {bracket.name + (mapOrder + 1)}
        </div >
        <div class="w-full p-2">
            <div>
                <div class="flex justify-between items-center">
                    <a class="lg:flex lg:flex-col" href={link}>
                        <h1 class="text-xl font-bold">{map.title}</h1>
                        <div class="text-lg font-semibold">
                            {map.artistName} / {creator.username}
                        </div>
                    </a>
                    <div class="bg-white h-12 aspect-square"></div>
                </div>
                <div class="flex flex-col lg:flex-row justify-between items-center">
                    <h2 class="font-bold bg-accent-400 text-bg p-1 rounded-md mt-5 lg:my-2">
                        {map.difficultyName}
                    </h2>
                    <div class="flex items-center justify-between gap-2 lg:gap-3 pt-5 lg:pt-0">
                        <span><b>★</b> {round2(difficulty.stars)}</span>
                        <span><b>Length</b> {formatLength(difficulty.totalLength)}</span>
                        <div class="flex items-center gap-2">
                            <span><b>CS</b> {round1(difficulty.cs)}</span>
                            <span><b>AR</b> {round1(difficulty.ar)}</span>
                            <span><b>HP</b> {round1(difficulty.hp)}</span>
                            <span><b>OD</b> {round1(difficulty.od)}</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div >
}