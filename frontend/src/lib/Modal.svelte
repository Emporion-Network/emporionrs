<script lang="ts">
  import type { Snippet } from "svelte";
  import { trapFocus } from "./actions.svelte";
  let {
    children,
    open = $bindable()
  }:{
    children?:Snippet<[]>
    open:boolean,
  } = $props();

  const closeOnEscape = (e:KeyboardEvent)=>{
    if (['Esc', 'Escape'].includes(e.key)) open = false;
  }

</script>

<svelte:window onkeydown={closeOnEscape}></svelte:window>

{#if open}
<div class="modal" onclick={()=>open = false} role="presentation" use:trapFocus>
    <div class="content" onclick={e => e.stopPropagation()} role="presentation">
        {@render children?.()}
    </div>
</div>
{/if}

<style lang="scss">
    @use "../mixins" as m;

    .modal{
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        display: flex;
        justify-content: center;
        align-items: center;
        display: flex;
        background-color: var(--neutral-a1);
        backdrop-filter: blur(1.4px);
        z-index: 99;
        @keyframes slideup {
            from {
                transform: translateY(100%);
                opacity: 0;
            }
            to {
                transform: translateY(0%);
                opacity: 1;
            }
        }
        .content{
            animation: slideup 200ms ease-out forwards;
            background-color: var(--neutral-1);
            padding: 1rem;
            border: 1px ;
            border: 1px solid var(--neutral-8);
            border-radius: 2px;
            @include m.media("<=phone"){
                width: 100vw;
                height: 80vh;
                position: absolute;
                bottom: 0;
            }
        }
    }
</style>
