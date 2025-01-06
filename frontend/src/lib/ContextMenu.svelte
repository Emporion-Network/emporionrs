<script lang="ts">
    import type { Snippet } from "svelte";
    let {
        opener,
        options,
    }: {
        opener: Snippet<[{ref:HTMLElement}]>;
        options: Snippet<[()=>true]>;
    } = $props();
    let pos = $state(new DOMRect());
    const onf = ()=>{
        pos = el.ref.getBoundingClientRect()
    }

    let el:{
        ref:HTMLElement,
        onfocus:()=>void,
    } = $state({ref:null!, onfocus:onf});
    
    const close = ()=>{
        //@ts-ignore
        document.activeElement?.blur();
        return true as const;
    }
</script>

<svelte:window onscroll={onf} onresize={onf}></svelte:window>

<div class="context-menu">
    {@render opener(el)}
    <div class="content"  style="--l:{pos.left}px; --t:{pos.top + pos.height}px; --w:{pos.width}px">
        {@render options(close)}
    </div>
</div>


<style lang="scss">
    .context-menu{
        display: contents;
        &:focus-within{
            .content{
                display: flex;
                flex-direction: column;
            }
        }
        .content{
            display: none;
            position: fixed;
            left: var(--l);
            top: var(--t);
            background-color: var(--parent-bg);
            border: 1px solid var(--neutral-6);
            border-radius: 3px;
            z-index: 1;
        }
    }
</style>
