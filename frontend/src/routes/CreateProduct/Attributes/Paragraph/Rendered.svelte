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

<p>{attributes[0]?.value?.[selectedLang]}</p>

<style lang="scss">
    p{
        text-align: center;
        white-space: pre-wrap;
        color: var(--neutral-11);
    }
</style>
