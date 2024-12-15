<script lang="ts">
    import Input from "../../lib/Input.svelte";
    let {
        value=$bindable(),
    }:{
        value:number
    } = $props()
</script>

<Input type="number" bind:value placeholder="" label="">
    <button class="max">max</button>
</Input>