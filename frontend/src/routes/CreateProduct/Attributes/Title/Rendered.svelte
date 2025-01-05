<script lang="ts">
    import type { SupportedLanguage } from "../../../../stores/translate.svelte";
    import type { Attribute } from "./_meta";
    let {
        attributes,
        selectedLang,
        pref = $bindable(),
    }: {
        attributes: Attribute[];
        selectedLang: SupportedLanguage;
        value: number;
        pref: number[];
        prefs: number[][];
        onupdate:(v:number[])=>void;
    } = $props();

</script>

<h2>{attributes[0]?.value?.[selectedLang]}</h2>

<style lang="scss">
    h2{
        text-align: center;
    }
</style>
