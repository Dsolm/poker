<script>
  import Card from "./Card.svelte";
</script>

<main>
  <div class="content">
    <h1>Poker</h1>
    <p>Player deck</p>
    <div class="cards">
      <Card rank="1" type="club"/>
      <Card rank="1" type="diamond"/>
      <Card rank="13" type="heart"/>
      <Card rank='8' type='spade'/>
    </div>
    <div class="cards">
      <Card rank="9" type="spade"/>
      <Card rank="1" type="diamond"/>
      <Card rank="11" type="heart"/>
      <Card rank='12' type='spade'/>
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
