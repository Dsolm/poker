<script>
  import Card from "./Card.svelte";
</script>

<main>
  <div class="content">
    <h1>Poker</h1>
    <p>Player deck</p>
    <div class="cards">
      <Card rank="ace" type="club" color="black" />
      <Card rank="ace" type="diamond" color="red" />
      <Card rank="king" type="heart" color="red" />
      <Card rank='8' type='spade' color='black'/>
    </div>
    <div class="cards">
      <Card rank="9" type="spade" color="black" />
      <Card rank="ace" type="diamond" color="red" />
      <Card rank="jack" type="heart" color="red" />
      <Card rank='queen' type='spade' color='black'/>
    </div>
  </div>
</main>

<style>
  .content {
    display: flex;
    min-height: 100vh;
    line-height: 1.1;
    text-align: center;
    flex-direction: column;
    justify-content: center;
  }

  .content h1 {
    font-size: 3.6rem;
    font-weight: 700;
  }

  .content p {
    font-size: 1.2rem;
    font-weight: 400;
    opacity: 0.5;
  }

  .cards{
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>
