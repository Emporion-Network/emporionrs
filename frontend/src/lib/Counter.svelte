<script lang="ts">
  let count: number = $state(0);
  import {getTranslator } from "../stores/translate.svelte";
  let t = getTranslator();
  const increment = () => {
    console.log(t.update('fa'));
    count += 1
  }
</script>

<button onclick={increment}>
  {t.t("weak_cuddly_dove_sail")} {count}
  {t.t("sea_actual_cobra_hunt")}
  {t.t("weak_teary_penguin_fry")}
  {t.t("sleek_topical_tadpole_tap")}
</button>
