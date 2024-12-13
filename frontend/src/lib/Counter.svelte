<script lang="ts">
  import { getLocation } from "../stores/location.svelte";
  import { getTheme } from "../stores/theme.svelte";
  let {
    path
  }:{
    path:string,
  } = $props()

  let location = getLocation();
  let theme = getTheme();
  const increment = () => {
    location.goTo(path);
  };
</script>

<button onclick={increment}>
  {path}
</button>

<style lang="scss"></style>
