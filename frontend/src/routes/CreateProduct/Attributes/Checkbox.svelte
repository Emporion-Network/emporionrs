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
        translatableKeys: ["title", "description"],
        isDefault: (a:any) => a.title === "" && a.description === "",
    };
</script>
<script lang="ts">
    import Input from "../../../lib/Input.svelte";
    import TranslatableInput from "../../../lib/TranslatableInput.svelte";
    import { translatedString, type SupportedLanguage, type T } from "../../../stores/translate.svelte";
    let { attribute = $bindable(), lang = $bindable() }: {
        attribute:Attribute,
        lang:SupportedLanguage,
    } = $props();
    
</script>
<div class="input-attribute">
    <TranslatableInput type="text" label="Attribute title" bind:selectedLang={lang} bind:value={attribute.title}/>
    <TranslatableInput type="textarea" label="Description" bind:selectedLang={lang} bind:value={attribute.description}/>
</div>