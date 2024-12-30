<script lang="ts" module>
    export const type = "select" as const;
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
        label:"close_legal_crossbill_seek"
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
    <TranslatableInput
        type="text"
        label={t.t("gaudy_home_emu_dust")}
        bind:value={attribute.title}
        selectedLang={lang}
    />
    <TranslatableInput
        type="text"
        label={t.t("due_bald_nuthatch_attend")}
        bind:value={attribute.value}
        selectedLang={lang}
    />
</div>