<script lang="ts">
    let value = $state("");
    export {value as v}
</script>
<div>
    {value}
    <input type="text" bind:value />
</div>