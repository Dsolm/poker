<script>
let { rank, type, onmousedown, draggable } = $props();
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
		 width: 20vh;
		 height: 30vh;
		 border: aliceblue;
		 margin: 1vh 0.5vw;
 }
</style>

