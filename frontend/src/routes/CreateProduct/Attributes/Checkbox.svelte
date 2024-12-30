<script lang="ts" module>
    export const type = "checkbox" as const;
    export const defaultAttribute = {
        type: type as typeof type,
        title: translatedString(),
        description: translatedString(),
    };
    export type Attribute = {
        type:typeof type,
        title:T<string>,
        description:T<string>,
    };
    export const meta = {
        type: type,
        defaultAttribute,
        label:"ago_top_stingray_pop"
    };
</script>
<script lang="ts">
    import TranslatableInput from "../../../lib/TranslatableInput.svelte";
    import { getTranslator, translatedString, type SupportedLanguage, type T } from "../../../stores/translate.svelte";
    let { attribute = $bindable(), lang = $bindable() }: {
        attribute:Attribute,
        lang:SupportedLanguage,
    } = $props();

    let t = getTranslator();
    
</script>
<div class="input-attribute">
    <TranslatableInput type="text" label={t.t("gaudy_home_emu_dust")} bind:selectedLang={lang} bind:value={attribute.title}/>
    <TranslatableInput type="textarea" label={t.t("flat_alive_porpoise_read")} bind:selectedLang={lang} bind:value={attribute.description}/>
</div>