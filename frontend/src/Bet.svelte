<script>
let { bet = $bindable(0) } = $props();
let temp_bet = $state(5);

function goAllIn() {
	alert("NOT IMPLEMENTED");
}

function formatCurrency(value) {
	const numberValue = parseFloat(value.replace(/[^0-9.]/g, ""));
	return isNaN(numberValue) ? "" : `$${numberValue.toFixed(2)}`;
}

function handleInput(event) {
	temp_bet = formatCurrency(event.target.value);
	/* event.target.value = temp_bet; */
	console.log(event.target.value);
}
</script>

<section class="bet">
	<div class="moneyinput">
		<input oninput={handleInput} type="number" value="10" />
		<span>€</span>
	</div>

	<button onclick={goAllIn}>All In</button>
	<button disabled={bet ? false : true} onclick={() => bet = temp_bet}>Bet</button>
</section>
	
<style>
 h1 {

		 margin: 0px;
		 text-align: center;
		 font-size: 2rem;
		 font-weight: 400;
 }


 .bet {
		 width: 100%;
		 height: 100%;
		 display: flex;
		 justify-content: center;
		 align-items: center;

		 gap: 8px;

		 input::-webkit-outer-spin-button,
		 input::-webkit-inner-spin-button {
				 /* display: none; <- Crashes Chrome on hover */
				 -webkit-appearance: none;
				 margin: 0px; /* <-- Apparently some margin are still there even though it's hidden */
		 }

		 input:focus {
				 outline: none;
		 }
		 input[type=number] {
				 -moz-appearance:textfield; /* Firefox */
				 margin: 0px;
		 }
		 
		 .moneyinput {
				 display: flex;
				 align-items: center;
				 justify-content: center;
				 overflow: visible;
				 
				 user-select: none;
				 -moz-user-select: none;
				 -khtml-user-select: none;
				 -webkit-user-select: none;
				 -o-user-select: none;
				 
				 input {

						 user-select: none;
						 -moz-user-select: none;
						 -khtml-user-select: none;
						 -webkit-user-select: none;
						 -o-user-select: none;
						 
						 flex-shrink: 1;
						 appearance: none;
						 background-color: transparent;
						 border: none;

						 margin: 0px;

						 padding: 13px 0px;
						 
						 color: #111;
						 text-align: end;
						 width: 80px;
						 

				 }
				 span {

						 user-select: none;
						 -moz-user-select: none;
						 -khtml-user-select: none;
						 -webkit-user-select: none;
						 -o-user-select: none;

						 padding: 13px 2px;
						 padding-right: 8px;
				 }

				 width: 100px;
				 
				 text-align: start;
				 font-size: 2.0em;

				 background-color: aliceblue;			 
				 border-radius: 8px;
				 border-style: none;
				 box-shadow: rgba(235, 235, 235, .15) 0 4px 9px;
				 color: #111;
				 
				 overflow: hidden;

				 white-space: nowrap;
		 }
 }

 
</style>
