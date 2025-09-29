<script>
import Bet from "./Bet.svelte";
import Game from "./Game.svelte";
import Tablero from "./Tablero.svelte";

let bet = $state(null);



</script>

<main>
<!--	{#if bet}
		<Game />
	{:else}
		<Bet bind:value={bet}/>
	{/if}-->
	<Game />
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
</style>