// <TournamentInfo {tournament} {rankRestrictions} {countryRestrictions} />

import { For, Index, JSX, Match, Show } from "solid-js";
import { Tournament } from "../lib/api/v1types";
import { Component } from "../lib/types";
import Flag from "./Flag";

import styles from "TournamentInfo.module.css";

//
const TournamentInfo: Component<{ tournament: Tournament }> = (props) => {
	const tournament = props.tournament;
	const rankRestrictions = tournament.rankRestrictions;
	const countries = tournament.countryRestrictions;

	let rankRestrictionElement: JSX.Element;
	if (rankRestrictions.length == 0) {
		rankRestrictionElement = <>Open Rank</>;
	} else if (rankRestrictions.length == 1) {
		rankRestrictionElement = (
			<>
				<span>{rankRestrictions[0].min}</span>
				<span class="px-1">-</span>
				<span>{rankRestrictions[0].max}</span>
			</>
		);
	} else {
		rankRestrictionElement = (
			<table class="min-w-full">
				<Index each={rankRestrictions}>
					{(range, i) => (
						<tr>
							<td class="font-bold">Tier {i + 1}:</td>
							<td class="pl-3">{range().min}</td>
							<td class="px-1">-</td>
							<td class="">{range().max}</td>
						</tr>
					)}
				</Index>
			</table>
		);
	}

	const styles = {
		infoHeading: "text-2xl font-bold text-left pr-10 lg:text-right",
		infoContent: "text-xl pl-10 text-left lg:text-left",
	};

	return (
		<>
			<div class="flex flex-col justify-center items-center rounded-2xl lg:rounded-lg p-3">
				<h1 class="text-5xl lg:text-6xl font-bold text-center p-3 pb-5">
					{tournament.name}
				</h1>
				<div class="flex [&>*]:py-3 [&>*]:flex-[0_1_40%] justify-center flex-wrap w-1/2">
					{/* Rank Ranges */}
					<div class={styles.infoHeading}>
						Rank Range{rankRestrictions.length > 1 ? "s" : ""}
					</div>
					<div class={styles.infoContent}>{rankRestrictionElement}</div>

					{/* BWS */}
					<Show when={rankRestrictions.length == 0}>
						<div class={styles.infoHeading}>BWS</div>
						<div class={styles.infoContent}>
							{tournament.bws ? "Yes" : "No"}
						</div>
					</Show>

					{/* Format */}
					<div class={styles.infoHeading}>Match Format</div>
					<div class={styles.infoContent}>
						<Show when={tournament.format > 0}>
							{tournament.format}v{tournament.format}
						</Show>
					</div>

					{/* Country Restrictions */}
					<Show when={countries.length > 0}>
						<div class={styles.infoHeading}>Country Restrictions</div>
						<div class={styles.infoContent}>
							<For each={countries}>
								{(country) => <Flag country={country} />}
							</For>
						</div>
					</Show>
				</div>
			</div>
		</>
	);
};

export default TournamentInfo;
