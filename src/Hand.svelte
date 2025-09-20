<script>
import Card from "./Card.svelte";
let cards = $state([]);
for (let i = 1; i <= 5; i++) {
	cards.push({ rank: i, type: "heart" });
}
</script>

<div>
	{#each cards as card}
		<Card {...card} />
	{/each}
</div>

<style>
 div {
		 pointer-events: none;
		 /* Make non-selectable */
		 user-select: none;
         -moz-user-select: none;
         -khtml-user-select: none;
         -webkit-user-select: none;
         -o-user-select: none;
		 /***********************/
		 
		 width: 100%;
		 diplay: flex;
		 gap: 400px;
		 position: absolute;
		 bottom: 0%;
 }

</style>
