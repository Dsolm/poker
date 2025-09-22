<script>
 import Card from "./Card.svelte";
 import Hand from "./Hand.svelte";

 let money = $state(0);
 let bet = $state(null);

 function computeScore(deck) {
	 let score = 0;
	 let aces = 0;
	 console.log(deck);
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
		 console.log(score + 11);
		 if (score + 11 <= 21) {
			 score += 11;
		 } else {
			 score += 1;
		 }
	 }
	 return score;
 }

 function randomCard() {
	 return {
		 rank: Math.floor(Math.random() * 13 + 1),
		 type: "heart",
	 };
 }

 let dealerDeck = $state([randomCard(), randomCard()]);
 let playerDeck = $state([randomCard(), randomCard()]);

 const dealerScore = $derived.by(() => computeScore(dealerDeck));
 const playerScore = $derived.by(() => computeScore(playerDeck));

 let result = $state(null);

 function restart() {
	 setTimeout(() => {
		 bet = null;
		 dealerDeck = [randomCard(), randomCard()];
		 playerDeck = [randomCard(), randomCard()];
		 result = null;
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
				 money += bet * 2;
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
					 money += bet * 3;
				 } else {
					 message = "Player wins!";
					 money += bet * 2;
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

<main>
	<div class="deck">
		<Hand bind:cards={dealerDeck}/>
		<p>Dealer: {dealerScore}</p>
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
		<p>You: {playerScore}</p>
		<Hand bind:cards={playerDeck}/>
	</div>
</main> 

<style>
 main {
		 margin-top: 2sh;
		 margin-bottom: 2sh;
		 
		 width: min(100vw, 1920px);
		 display: flex;
		 height: 100svh;
		 line-height: 1.1;
		 flex-direction: column;
		 justify-content: space-evenly;
		 align-items: center;
 }

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

 button {
		 appearance: none;
		 backface-visibility: hidden;
		 background-color: #27ae60;
		 border-radius: 8px;
		 border-style: none;
		 box-shadow: rgba(39, 174, 96, .15) 0 4px 9px;
		 box-sizing: border-box;
		 color: #fff;
		 cursor: pointer;
		 display: inline-block;
		 font-family: Inter,-apple-system,system-ui,"Segoe UI",Helvetica,Arial,sans-serif;
		 font-size: 16px;
		 font-weight: 600;
		 letter-spacing: normal;
		 line-height: 1.5;
		 outline: none;
		 overflow: hidden;
		 padding: 13px 20px;
		 position: relative;
		 text-align: center;
		 text-decoration: none;
		 transform: translate3d(0, 0, 0);
		 transition: all .3s;
		 user-select: none;
		 -webkit-user-select: none;
		 touch-action: manipulation;
		 vertical-align: top;
		 white-space: nowrap;
 }

 .content p {
		 margin: 0px;
		 text-align: center;
		 font-size: 1.2rem;
		 font-weight: 400;
		 opacity: 0.5;
 }
</style>
