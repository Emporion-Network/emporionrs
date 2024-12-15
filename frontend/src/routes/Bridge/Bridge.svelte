<script lang="ts">
    import Input from "../../lib/Input.svelte";
    import Route from "../../lib/Route.svelte";
    import { getTranslator } from "../../stores/translate.svelte";
    import { chains, ibc, assets } from "chain-registry";
    import ChainSelector from "./ChainSelector.svelte";
    import Amount from "./Amount.svelte";
    type Chain = (typeof chains)[0];
    const supportedChains = chains.filter((e) => {
        return e.chain_type === "cosmos" && e.network_type == "mainnet";
    });
    let from = $state(chains.find((e) => e.chain_name == "cosmoshub")!);
    let to = $state(chains.find((e) => e.chain_name == "osmosis")!);
    let amout = $state(0);

    let paths = $derived.by(() => {
        let a = assets.find(a => a.chain_name == from.chain_name)?.assets.filter(
            a => a.type_asset === "sdk.coin" || a.type_asset === "cw20");

        return a;
    });
    const flip = ()=>{
        let r = from;
        from = to;
        to = r;
    }


    let t = getTranslator();
</script>


<Route path="/transfer">
    <div class="bridge">
        <div class="ctr">
            <h1>{t.t("light_giant_shell_attend")}</h1>
            <h3>{t.t("stale_minor_cuckoo_type")}</h3>
            <ChainSelector bind:value={from} {supportedChains}/>
            <button onclick={flip} class="swap" aria-label="{t.t("nice_mushy_ox_vent")}">
                <i class="ri-arrow-up-down-line"></i>
            </button>
            <h3>{t.t("minor_orange_hawk_honor")}</h3>
            <ChainSelector bind:value={to} {supportedChains}/>
            <Amount bind:value={amout}/>
            {JSON.stringify(paths?.map(a => [a.symbol, a.deprecated]), null, 4)}
        </div>
    </div>
</Route>

<style lang="scss">
    .bridge{
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        .ctr{
            width: 100%;
            max-width: 600px;
            --bg-color:var(--neutral-2);
            background-color: var(--bg-color);
            padding: 1rem;
            border: 1px solid var(--neutral-6);
            border-radius: 3px;
            display: flex;
            flex-direction: column;
            .swap{
                width: 40px;
                height: 40px;
                align-self: center;
                margin-top: 1rem;
                margin-bottom: -2rem;
                font-size: 1.4rem;
                outline: none;
                background-color: transparent;
                color: var(--neutral-11);
                border: none;
                cursor: pointer;
                &:hover{
                    color: var(--neutral-12);
                }
            }
            h3{
                color: var(--neutral-11);
                margin-top: 1rem;
                font-size: 0.8rem;
            }
        }
    }
</style>
