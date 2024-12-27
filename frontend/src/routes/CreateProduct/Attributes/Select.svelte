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
        isDefault: (a: any) => a.value === "" && a.title === "",
        translatableKeys: ["title", "value"],
    };

</script>
<script lang="ts">
    import Input from "../../../lib/Input.svelte";
    import { translatedString, type SupportedLanguage, type T } from "../../../stores/translate.svelte";
    let { attribute = $bindable(), lang = $bindable() }: {
        attribute:Attribute,
        lang:SupportedLanguage,
    } = $props();
</script>

<div class="input-attribute">
    <Input
        type="text"
        label="Attribute title"
        placeholder="Attribute title"
        bind:value={attribute.title[lang]}
    />
    <Input
        type="text"
        label="Option value"
        placeholder="Option value"
        bind:value={attribute.value[lang]}
    />
</div>