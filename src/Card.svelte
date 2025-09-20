<script>
    let {rank, type} = $props();
</script>

<img src={`./images/card_${rank}_${type}.png`} alt="${rank} of ${type}s">
<style>
    img{
        height: 15%;
        width: 10%;
        border-color: aliceblue;
    }
</style>

