<script>
import Bet from "./Bet.svelte";
import Card from "./Card.svelte";
import Hand from "./Hand.svelte";

 console.log("Negar!");
 
 function computeScore(deck) {
	 let score = 0;
	 let aces = 0;

	 for (const c of deck) {
		 if (c.rank >= 2 && c.rank <= 10) {
			 score += c.rank;
		 } else if (c.rank === 1) {
			 aces += 1;
		 } else {
			 score += 10;
		 }
	 }

	 for (let i = 0; i < aces; i++) {
		 if (score + 11 <= 21) {
			 score += 11;
		 } else {
			 score += 1;
		 }
	 }
	 return score;
 }

 function randomCard() {
	 const suits = ["heart", "spade", "club", "diamond"];
	 return {
		 rank: Math.floor(Math.random() * 13 + 1),
		 type: suits[Math.floor(Math.random() * suits.length)],
	 };
}

let dealerDeck = $state([randomCard(), randomCard()]);
let playerDeck = $state([randomCard(), randomCard()]);

const dealerScore = $derived.by(() => computeScore(dealerDeck));
const playerScore = $derived.by(() => computeScore(playerDeck));

let result = $state(null);

if (dealerScore == 21 && playerScore == 21) {
	result = new Promise((fulfill, reject) => {
		fulfill({ winner: "Standoff", message: "Standoff" });
	});
	restart();
} else if (dealerScore == 21) {
	result = new Promise((fulfill, reject) => {
		fulfill({ winner: "Dealer", message: "Black jack! Dealer wins" });
	});
	restart();
} else if (playerScore == 21) {
	result = new Promise((fulfill, reject) => {
		fulfill({ winner: "Player", message: "Black jack! Player wins" });
	});
	restart();
}

function restart() {
	setTimeout(() => {
		dealerDeck = [];
		playerDeck = [];
		setTimeout(() => {
			dealerDeck = [randomCard(), randomCard()];
			playerDeck = [randomCard(), randomCard()];
			result = null;
		}, 1000);
	}, 1000);
}

function hit() {
	playerDeck.push(randomCard());
	if (playerScore === 21) {
		stand();
	} else if (playerScore > 21) {
		result = new Promise((fulfill, reject) => {
			fulfill({ winner: "Dealer", message: "Player bust! Dealer wins." });
		});
		restart();
	}
}

function checkWinner() {
	result = new Promise((fulfill, reject) => {
		setTimeout(() => {
			let message = null;
			if (dealerScore > 21) {
				message = "Dealer bust!";
			} else if (dealerScore === playerScore) {
				message = "Standoff!";
			} else if (dealerScore > playerScore) {
				if (dealerScore === 21 && dealerDeck.length === 2) {
					message = "Black jack. Dealer wins!";
				} else {
					message = "Dealer wins!";
				}
			} else if (playerScore > dealerScore) {
				if (playerScore === 21 && playerDeck.length === 2) {
					message = "Black jack. Player wins!";
				} else {
					message = "Player wins!";
				}
			}
			restart();
			fulfill({ message });
		}, 0);
	});
}

function stand() {
	var id = setInterval(() => {
		runDealer();
		if (dealerScore >= 17) {
			clearInterval(id);
			checkWinner();
		}
	}, 500);
}

function runDealer() {
	if (dealerScore < 17) {
		dealerDeck.push(randomCard());
	}
}
</script>


<div class="deck">
	<Hand bind:cards={dealerDeck}/>
	{#if dealerScore > 0}
		<p>Dealer: {dealerScore}</p>
	{/if}
</div>

<div class="center">

	{#if result}
		{#await result then res}
			<h1>{res.message}</h1>
		{/await}

		{:else}
		<button onclick={hit}>
			Hit
		</button>
		<button onclick={stand}>
			Stand
		</button>
	{/if}
</div>


<div class="deck">
	{#if playerScore}
		<p>You: {playerScore}</p>
	{/if}
	<Hand bind:cards={playerDeck}/>
</div>



<style>
 .deck {
		 max-width: 100%;
 }
 
 .deck p {
		 display: flex;
		 justify-content: center;
 }

 
 .center {
		 flex-shrink: 1;
		 user-select: none;
         -moz-user-select: none;
		 -khtml-user-select: none;
		 -webkit-user-select: none;
		 -o-user-select: none;
		 display: flex;
		 justify-content: center;
		 gap: 10px;
 }

 .content p {
		 margin: 0px;
		 text-align: center;
		 font-size: 1.2rem;
		 font-weight: 400;
		 opacity: 0.5;
 }
</style>
