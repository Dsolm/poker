<script>
let { rank, type } = $props();
</script>

<img src={`./images/card_${rank}_${type}.png`} alt="${rank} of ${type}s">
<style>
 img {
		 pointer-events: none;
		 /* Make non-selectable */
		 user-select: none;
		 -moz-user-select: none;
		 -khtml-user-select: none;
		 -webkit-user-select: none;
		 -o-user-select: none;
		 /***********************/
		 
		 width: 10vw;
		 border-color: aliceblue;
		 margin: 1vh 0.5vw;
 }
</style>

