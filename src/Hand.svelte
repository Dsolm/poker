<script>
const { draggable, cards = $bindable([]) } = $props();

import Card from "./Card.svelte";
import DraggableCard from "./DraggableCard.svelte";

const suitsOrder = {
	club: 0,
	diamond: 1,
	heart: 2,
	spade: 3,
};

function compareCards(cardA, cardB) {
	const suitA = suitsOrder[cardA.type];
	const suitB = suitsOrder[cardB.type];

	if (suitA !== suitB) {
		return suitA - suitB;
	}
	return cardA.rank - cardB.rank;
}

if (!cards) {
	for (let i = 1; i <= 5; i++) {
		cards.push({ rank: i, type: "spade" });
	}
	cards.sort(compareCards);
}

let selected = $state(null);

let y = $state(0);
let x = $state(0);

function handleMouseDown(event) {
	x = event.clientX;
	y = event.clientY;
	document.addEventListener("mousemove", handleMouseMove);
	document.addEventListener("mouseup", handleMouseUp, { once: true });
}

function handleMouseMove(event) {
	x = event.clientX;
	y = event.clientY;
}

function handleMouseUp(event) {
	cards.push(selected);
	selected = null;
	cards.sort(compareCards);
	document.removeEventListener("mousemove", handleMouseMove);
}
</script>

{#if selected}
	<DraggableCard
		x="{x}px"
		y="{y}px" 
		{selected}
	/>
{/if}

<div>
	{#each cards as card, idx (idx)}
		<Card
			{...card}
			{draggable}
			onmousedown={draggable ? (e) => {
					selected = card;
					cards.splice(idx, 1);
					handleMouseDown(e);
				} : null}
			/>
	{/each}
</div>

<style>
 div {
		 /* Make non-selectable */
		 /* user-select: none;
			-moz-user-select: none;
			-khtml-user-select: none;
			-webkit-user-select: none;
			-o-user-select: none; */
		 /***********************/
		 width: 100%;
		 display: flex;
		 flex-direction: row;
		 overflow-x: hidden;
 }

</style> 