<script>
let { rank, type, onmousedown } = $props();
</script>

<img
	draggable="false"
	src="./images/card_{rank}_{type}.png" alt="{rank} of {type}s"
	{onmousedown}
/>
<style>
 img {
		 /* Change cursor icon to grabbing hand. */
		 cursor: grab;
		 /* Make non-selectable */
		 user-select: none;
		 -moz-user-select: none;
		 -khtml-user-select: none;
		 -webkit-user-select: none;
		 -o-user-select: none;
		 /***********************/
		 width: 10vw;
		 border: aliceblue;
		 margin: 1vh 0.5vw;
 }
</style>

