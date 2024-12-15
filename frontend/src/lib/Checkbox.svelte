<script lang="ts">
  let {
    value=$bindable(),
    tabindex=0,
    readonly,
  }:{
    value:boolean,
    tabindex?:number,
    readonly?:boolean,
  } = $props();

  const toggle = ()=>{
    if(readonly) return;
    value = !value;
  }
</script>

<button class:selected={value} class:readonly onclick={toggle} aria-label={value ? "on" : "off"} {tabindex}>
    {#if value}
        <i class="ri-check-line"></i>
    {/if}
</button>

<style lang="scss">
    button{
        width: 1rem;
        height: 1rem;
        aspect-ratio: 1/1;
        display: flex;
        justify-content: center;
        align-items: center;
        border: 1px solid var(--neutral-7);
        background-color: transparent;
        cursor: pointer;
        outline: none;
        font-weight: 900;
        &.readonly{
          cursor: default;
        }
        &:hover, &:focus-visible{
            border: 1px solid var(--main-10);
            background-color: var(--main-8);
        }
        &.selected{
            background-color: var(--main-10);
            color: var(--main-1);
            border: 1px solid var(--main-10);
            &:hover, &:focus-visible{
              background-color: var(--main-8);
            }
        }
    }
</style>
