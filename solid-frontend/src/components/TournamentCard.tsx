import { Component, createSignal } from "solid-js";
import { Tournament } from "../lib/api/v1types";

const TournamentCard: Component<{ tournament: Tournament }> = (props) => {
	const tournament = () => props.tournament;
	const rankRestrictions = tournament().rankRestrictions.filter(
		(r) => r !== undefined,
	);

	const [rankRange, setRankRange] = createSignal("");

	const first = rankRestrictions[0];
	const last = rankRestrictions[rankRestrictions.length - 1];

	if (rankRestrictions.length === 0) {
		setRankRange("Open Rank");
	} else if (rankRestrictions.length === 1) {
		setRankRange(`${first?.min} - ${first?.max}`);
	} else {
		setRankRange(`Tiered ${first?.min} - ${last?.max}`);
	}

	if (rankRange() !== "Open Rank") {
		setRankRange(
			(r) => `${r} ${tournament().bws ? "with BWS" : "without BWS"}`,
		);
	}

	return (
		<>
			<div class="bg-bg-400 shadow-md shadow-bg-100 text-gray-200 rounded-2xl lg:rounded-lg min-w-full min-h-full transition-all duration-200 hover:scale-105">
				<a href="/tournament/{tournament.id}">
					<img
						class="object-cover rounded-t-xl lg:rounded-t-lg w-full lg:h-32"
						alt="banner"
						src="https://i.ppy.sh/f49c7b14d7308ee720c1dbd9b6c9d78dd1469a2d/68747470733a2f2f692e6962622e636f2f7a5a6a705076372f6d72656b6b2d6772616e646d61737465722d6375702d62616e6e65722d424554412d4c4f474f2e706e67"
					/>
					<div class="p-4">
						<h1 class="text-4xl lg:text-3xl font-bold text-center pb-2">
							{tournament().name}
						</h1>
						<div class="p-2 lg:p-0">
							<div class="text-right text-lg lg:text-lg truncate">
								{rankRange()}
							</div>
							<div class="text-right text-lg truncate">
								{tournament().format}v{tournament().format}
							</div>
						</div>
					</div>
				</a>
			</div>
		</>
	);
};

export default TournamentCard;
