<script lang="ts">
  import { type ChangeEventHandler } from "svelte/elements";
  import { type Snippet } from "svelte";
  let {
    onchange,
    children,
  }: {
    children?: Snippet<[]>;
    onchange?:(files:File[])=>void
  } = $props();

  let inputEl: HTMLInputElement;

  const change: ChangeEventHandler<HTMLInputElement> = (evt) => {
    const newfiles = Array.from(evt.currentTarget.files || []);
    evt.currentTarget.value = "";
    onchange?.(newfiles);
  };
  const ondrop = (e: DragEvent) => {
    e.preventDefault();
    let files = [...(e.dataTransfer?.files || [])];
    onchange?.(files);
  };
  const ondragover = (e: Event) => {
    e.preventDefault();
  };
  const onclick = () => {
    inputEl.click();
  };
  const onkeydown = (e: KeyboardEvent) => {
    if (["Enter", " "].includes(e.key)) onclick();
  };
</script>

<div
  class="file-uploader"
  {ondrop}
  {ondragover}
  {onclick}
  {onkeydown}
  tabindex="0"
  role="button"
>
  <input
    bind:this={inputEl}
    onchange={change}
    type="file"
    multiple
    accept="image/*"
  />
  {#if children}
    {@render children?.()}
  {:else}
    <h1>Drag and drop photos here</h1>
    <button class="button-secondary">Or Add Photos</button>
  {/if}
</div>

<style lang="scss">
  h1 {
    text-align: center;
  }
  .file-uploader {
    min-height: 160px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    border: 1px solid var(--neutral-8);
    border-radius: 2px;
    background-color: var(--neutral-2);
    outline: none;
    position: relative;
    user-select: none;
    gap: 1rem;
    margin-top: 0.5rem;
    &:focus-within {
      border: 1px solid var(--main-10);
    }
    input {
      opacity: 0;
      pointer-events: none;
      position: absolute;
    }
  }
</style>
