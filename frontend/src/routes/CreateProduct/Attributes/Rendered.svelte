<script lang="ts">
    import { metas, type Attribute } from "./_metas";
    import { type SupportedLanguage } from "../../../stores/translate.svelte";
    import Buttons from "./Buttons/Rendered.svelte";
    import Checkbox from "./Checkbox/Rendered.svelte";
    import Select from "./Select/Rendered.svelte";
    import Color from "./Color/Rendered.svelte";
    import type { Product } from "../Form.svelte";
    import { intersect } from "../../../lib/utils";

    const map = {
        [metas.buttons.type]: Buttons,
        [metas.checkbox.type]: Checkbox,
        [metas.select.type]: Select,
        [metas.color.type]: Color,
    };

    let {
        products,
        selectedLang,
        selectedProductId = $bindable(),
    }: {
        selectedProductId: number;
        products: Product[];
        selectedLang: SupportedLanguage;
    } = $props();

    let attributes = $derived.by(() => {
        let ret: Attribute[][] = [];
        products.forEach((p) => {
            p.attributes.forEach((a, j) => {
                ret[j] = ret[j] ?? [];
                ret[j].push(a);
            });
        });
        return ret;
    });

    let pref: number[][] = $state([]);

    const get = (i: number) => () => {
        return pref[i];
    };
    const set = (i: number) => (v: number[]) => {
        pref[i] = v;
        const tgt = intersect(...pref);
        if (tgt.length === 0) {
            selectedProductId = v[0];
        } else if (!tgt.includes(selectedProductId)) {
            selectedProductId = tgt[0];
        }
    };
</script>

<div class="attributes">
    {#each attributes as attr, i}
        {@const Component = map[attr[0].display_type] as any}
        <Component
            bind:pref={get(i), set(i)}
            attributes={attributes[i]}
            selectedLang={selectedLang}
            value={selectedProductId}
            prefs={pref}
        />
    {/each}
</div>
