<script>
import Card from "./Card.svelte";
let { x, y, selected } = $props();
</script>

<div style="position: fixed; left: {x}; top: {y}">
	<Card {...selected}/>
</div>

<style>
 div {
		 /**
		  * Center arround mouse.
		  */
		 transform: translate(-50%, -50%);
 }
</style>