<script>
const { rank, type, onmousedown, draggable } = $props();
</script>

<img
	style="cursor: {draggable ? "grab" : ""}"
	draggable="false"
	src="./images/card_{rank}_{type}.png" alt="{rank} of {type}s"
	{onmousedown}
/>
<style>
 img {
		 /* Make non-selectable */
		 user-select: none;
		 -moz-user-select: none;
		 -khtml-user-select: none;
		 -webkit-user-select: none;
		 -o-user-select: none;
		 /***********************/
		 /* width: min(23vmin, 23%); */
		 max-height: 20svh;
		 /* width: 48%; */
		 border: aliceblue;
		 overflow-x: scroll;
		 margin: 1vh 0.5vw;
 }
</style>

