<script>
    let {rank, type, color} = $props();

</script>

<img src={`./images/card_${rank}_${type}_${color}.png`} alt="card_${rank}_${type}_${color}">
<style>
    img{
        height: 15%;
        width: 10%;
        border-color: aliceblue;
    }
</style>

