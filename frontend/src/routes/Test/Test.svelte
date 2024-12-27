<script lang="ts">
    import Route from "../../lib/Route.svelte";
    let x =  $state((v:string)=>{
        return `${v} Humm`
    });

    const onclick = ()=>{
        x = (v:string)=>{
            return `${v} Hooo`
        }
    }
</script>

<Route path="/test">
    <div>
        <button {onclick}>update</button>
        {x("hello")}
    </div>
</Route>

<style lang="scss">
</style>
