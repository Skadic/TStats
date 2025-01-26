import { For, Show } from "solid-js";
import { Stage } from "../lib/api/v1";
import { Component } from "../lib/types";

import { Accordion } from "@kobalte/core/accordion";


export const StageCard: Component<{stage: Stage}> = (props) => {
    const stage = props.stage;



	return (
		<div
			use:melt={$item(stage.name ?? 'unnamed stage')}
			class="flex-1 my-3 rounded-lg pb-4 bg-bg-400"
		>
			<button
				use:melt={$trigger(stage.name ?? 'unnamed stage')}
				class="w-full text-3xl font-bold flex p-2 rounded-lg bg-bg-500 hover:scale-105 transition-all"
			>
				{stage.name ?? 'unnamed stage'}
				{#if (stage.bestOf ?? -1) > 0}
					<h2 class="flex-1 text-right">Best of {stage.bestOf ?? -1}</h2>
				{/if}
			</button>

<Show when={}>
			{#if $value === stage.name ?? 'unnamed stage'}
				<div
					use:melt={$content(stage.name ?? 'unnamed stage')}
					transition:slide={{ duration: 200 }}
				>
					<StageCard
						tournamentId={tournament.key?.id ?? -1}
						stage={{
							name: stage.name ?? 'unnamed stage',
							bestOf: stage.bestOf ?? -1,
							stageOrder: i
						}}
					/>
				</div>
			{/if}

</Show>
		</div>
	);
};
