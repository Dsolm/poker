<script>
 import Card from "./Card.svelte";
 import Hand from "./Hand.svelte";

 function computeScore(deck) {
	 let score = 0;
	 let aces = 0;
	 console.log(deck);
	 for (let c of deck) {
		 if (c.rank >= 2 && c.rank <= 10) {
			 score += c.rank;
		 } else if (c.rank == 1) {
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

 let dealerScore = $derived.by(() => computeScore(dealerDeck));
 let playerScore = $derived.by(() => computeScore(playerDeck));

 let result = $state(null);

 function restart() {
	 setTimeout(() => {
		 dealerDeck = [randomCard(), randomCard()];
		 playerDeck = [randomCard(), randomCard()];
		 result = null;
	 }, 1000);
 }

 function hit() {
	 playerDeck.push(randomCard());
	 if (playerScore == 21) {
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
			 } else if (dealerScore == playerScore) {
				 message = "Standoff!";
			 } else if (dealerScore > playerScore) {
				 if (dealerScore == 21 && dealerDeck.length == 2) {
					 message = "Black jack. Dealer wins!";
				 } else {
					 message = "Dealer wins!";
				 }
			 } else if (playerScore > dealerScore) {
				 if (playerScore == 21 && playerDeck.length == 2) {
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

<main>
	<div class="content">
		<div>
			<Hand bind:cards={dealerDeck}/>
			<p>Dealer: {dealerScore}</p>
		</div>

		{#if result}
			{#await result then res}
				<h1>{res.message}</h1>
			{/await}
		{:else}
			<div>
				<button onclick={hit}>
					Hit
				</button>
				<button onclick={stand}>
					Stand
				</button>
			</div>
		{/if}
		

		<div>
			<p>You: {playerScore}</p>
			<Hand bind:cards={playerDeck}/>
		</div>
	</div>
</main>

<style>
 .content {
		 display: flex;
		 min-height: 100vh;
		 line-height: 1.1;
		 flex-direction: column;
		 justify-content: space-between;
		 align-items: center;
 }

 .content h1 {
		 font-size: 3.6rem;
		 font-weight: 700;
 }

 div {
		 user-select: none;
         -moz-user-select: none;
		 -khtml-user-select: none;
		 -webkit-user-select: none;
		 -o-user-select: none;
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

