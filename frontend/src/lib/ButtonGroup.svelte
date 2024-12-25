<script lang="ts">
  import type { Snippet } from "svelte";

  type T = $$Generic;
  let {
    value = $bindable(),
    options = $bindable(),
    optionRenderer
  }:{
    value:T
    options:T[],
    optionRenderer:Snippet<[T, boolean]>,
  } = $props();
  const select = (v:T)=>()=>{
    value = v;
  }
</script>

<div class="button-group">
    {#each options as option}
        {@const selected = value == option}
            <button onclick={select(option)} class:selected>
                {@render optionRenderer(option, selected)}
            </button>
    {/each}
</div>

<style lang="scss">
    .button-group{
        display: inline-flex;
        margin-top: 0.5rem;
        margin-bottom: 0.5rem;
        border: 1px solid var(--neutral-8);
        border-radius: 4px;
        &:hover{
            background-color: var(--neutral-2);
        }
    }


    button{
        cursor: pointer;
        border: 1px solid transparent;
        height: var(--height-1);
        color: var(--neutral-12);
        background-color: transparent;
        width: 100px;
        border-radius: 3px;
        position: relative;
        outline: none;
        &:focus-visible{
            border: 1px solid var(--main-10);
            &::after{
                display: none;
            }
        }
        &::after{
            content: "";
            height: 60%;
            width: 1px;
            left: 0;
            position: absolute;
            display: block;
            background-color:  var(--neutral-8);
            top:20%;
        }
        &:first-child::after{
            display: none;
        }
        &.selected + button::after{
            display: none;
        }
        &.selected{
            background-color: var(--neutral-3);
            &::after{
                display: none;
            }
        }
    }
</style>
