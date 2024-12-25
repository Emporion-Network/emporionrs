<script lang="ts" module>
    export const type = "select" as const;
    export const defaultAttribute = {
        type: type as typeof type,
        value: "",
        title: "",
    };
    export type Attribute = {
        type: typeof type;
        value: string;
        title: string;
    };

    export const meta = {
        type: type,
        defaultAttribute,
        isDefault: (a: Attribute) => a.value === "" && a.title === "",
        getTranslatables: (a: Attribute) => [a.title, a.value],
    };

</script>
<script lang="ts">
    import Input from "../../../lib/Input.svelte";
    let {
        value = $bindable(),
        title = $bindable(),
    }: Omit<Attribute, "type"> = $props();
</script>

<div class="input-attribute">
    <Input
        type="text"
        label="Attribute title"
        placeholder="Attribute title"
        bind:value={title}
    />
    <Input
        type="text"
        label="Option value"
        placeholder="Option value"
        bind:value
    />
</div>