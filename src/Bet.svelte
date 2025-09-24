<script>
const { bet = $bindable(0) } = $props();
const temp_bet = $state(0);

function goAllIn() {
	alert("NOT IMPLEMENTED");
}
</script>

<div>
	<input bind:value={temp_bet} type="number" />
	<button onclick={goAllIn}>All In</button>
	<button disabled={bet ? false : true} onclick={() => bet = temp_bet} >Play</button>
</div>

<style>
 div {
		 width: 100%;
		 height: 100%;
		 display: flex;
		 justify-content: center;
		 align-items: center;

		 gap: 8px;

		 input {
				 appearance: none;
				 backface-visibility: hidden;
				 background-color: aliceblue;
				 border-radius: 8px;
				 border-style: none;
				 box-shadow: rgba(235, 235, 235, .15) 0 4px 9px;
				 color: #111;
				 display: inline-block;
				 outline: none;
				 overflow: hidden;
				 padding: 13px 20px;
				 position: relative;
				 text-align: center;
				 text-decoration: none;
				 user-select: none;
				 -webkit-user-select: none;
				 touch-action: manipulation;
				 vertical-align: top;
				 white-space: nowrap;
		 }
		 
 }

 
</style>
