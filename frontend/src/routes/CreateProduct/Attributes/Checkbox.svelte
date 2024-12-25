<script lang="ts" module>
    export const type = "checkbox" as const;
    export const defaultAttribute = {
        type: type as typeof type,
        title: "",
        description: "",
    };
    export type Attribute = {
        type:typeof type,
        title:string,
        description:string,
    };
    export const meta = {
        type: type,
        defaultAttribute,
        isDefault: (a:Attribute) => a.title === "" && a.description === "",
        getTranslatables: (a:Attribute) => [a.title, a.description],
    };
</script>
<script lang="ts">
    import Input from "../../../lib/Input.svelte";
    let {
        title = $bindable(),
        description = $bindable(),
    }:Omit<Attribute, "type"> = $props();
</script>
<div class="input-attribute">
    <Input type="text" label="Attribute title" placeholder="Attribute title" bind:value={title}/>
    <Input type="textarea" label="Description" placeholder="Description" bind:value={description}/>
</div>