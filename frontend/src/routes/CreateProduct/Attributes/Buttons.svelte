<script lang="ts" module>
    export const type = "buttons" as const;
    export const defaultAttribute = {
        type: type as typeof type,
        value: translatedString(),
        title: translatedString(),
    };
    export type Attribute = {
        type: typeof type;
        value: T<string>;
        title: T<string>;
    };

    export const meta = {
        type: type,
        defaultAttribute,
        label:"sleek_vivid_koala_dare"
    };
</script>

<script lang="ts">
    import TranslatableInput from "../../../lib/TranslatableInput.svelte";
    import { getTranslator, translatedString, type SupportedLanguage, type T } from '../../../stores/translate.svelte';
    let { attribute = $bindable(), lang = $bindable() }: {
        attribute:Attribute,
        lang:SupportedLanguage,
    } = $props();
    let t = getTranslator();
</script>

<div class="input-attribute">
    <TranslatableInput
        type="text"
        label={t.t("gaudy_home_emu_dust")}
        bind:selectedLang={lang}
        bind:value={attribute.title}
    />
    <TranslatableInput
        type="text"
        label={t.t("pretty_any_husky_snip")}
        bind:selectedLang={lang}
        bind:value={attribute.value}
    />
</div>
