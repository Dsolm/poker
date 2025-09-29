<script>
  import Card from "./Card.svelte";

  let x = $state(0);
  let y = $state(0);

  function handleMouseDown(event) {
    x = event.clientX;
    y = event.clientY;
    console.log("HELLO");
    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp, { once: true });
  }

  function handleMouseMove(event) {
    x = event.clientX;
    y = event.clientY;
  }

  function handleMouseUp(event) {
    document.removeEventListener("mousemove", handleMouseMove);
  }
</script>

<div class="cards">
  <div class="carta" style="top:{y}; left:{x}" onclick={handleMouseDown}>
    <Card rank="1" type="heart" />
  </div>
</div>

<style>
  div {
    display: flex;
    justify-content: center;
  }
  .carta {
    background-color: yellow;
    z-index: 1;
    position: fixed;
  }
</style>
