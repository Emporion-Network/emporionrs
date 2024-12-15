<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import MultiSelect from "../../lib/MultiSelect.svelte";
    import { chains, ibc, assets } from "chain-registry";
    type Chain = (typeof chains)[0];
    const quickSelect = chains.filter(c => ["osmosis","cosmoshub","juno"].includes(c.chain_name));
    let {
        supportedChains,
        value = $bindable(),
    }: {
        supportedChains: Chain[];
        value: Chain;
    } = $props();
    let filter = $state("");
    const select = (v:Chain)=>()=>{
        value = v;
    }
</script>

{#snippet chain(v: Chain)}
    <div class="chain-option">
        {#if v.logo_URIs?.png || v.logo_URIs?.svg}
            <img
                src={v.logo_URIs?.png || v.logo_URIs?.svg}
                alt={v.pretty_name}
            />
        {:else}
            <i class="ri-copper-coin-fill"></i>
        {/if}
        <span>
            {v.pretty_name}
        </span>
    </div>
{/snippet}

<MultiSelect
    options={supportedChains}
    bind:value
    multiple={false}
    filter={(v) => v.chain_name.includes(filter)}
>
    {#snippet valueRenderer(v)}
        {@render chain(v)}
    {/snippet}
    {#snippet filterRenderer()}
        <div class="filter">
            <Input bind:value={filter} label="" placeholder="" type="search" />
            <div class="quick-select">
                {#each quickSelect as c}
                    <button onclick={select(c)}>
                        <img src="{c.logo_URIs?.png}" alt="{c.pretty_name}">
                        {c.pretty_name}
                    </button>
                {/each}
            </div>
        </div>
    {/snippet}
    {#snippet optionRenderer(v)}
        {@render chain(v)}
    {/snippet}
</MultiSelect>

<style lang="scss">
    .chain-option {
        display: flex;
        gap: 1rem;
        align-items: center;
        img {
            width: 30px;
            height: 30px;
        }
        i{
            font-size: 30px;
        }
    }
    .filter {
        :global(.input) {
            margin-top: 0;
            border: none;
            border-bottom: 1px solid var(--neutral-6);
            flex-direction: row-reverse;
        }
        .quick-select{
            display: flex;
            gap:0.5rem;
            padding: 0.5rem;
            border-bottom: 1px solid var(--neutral-6);
            button{
                display: flex;
                justify-content: center;
                align-items: center;
                background-color: transparent;
                outline: none;
                border: 1px solid var(--neutral-6);
                color: var(--neutral-12);
                padding: 0.5rem;
                padding-right: 1rem;
                border-radius: 5rem;
                font-size: 0.8rem;
                gap: 0.5rem;
                cursor: pointer;
                &:hover{
                    border: 1px solid var(--neutral-8);
                    background-color: var(--neutral-2);
                }
            }
            img{
                width: 1rem;
                height: 1rem;
            }
        }
    }
</style>
