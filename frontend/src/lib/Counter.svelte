<script lang="ts">
  let count: number = $state(0);
  import {getT} from "../stores/translate.svelte";
  let gt = getT();
  const increment = () => {
    console.log(gt.update('fa'));
    count += 1
  }
</script>

<button onclick={increment}>
  {gt.t("weak_cuddly_dove_sail")} {count}
  {gt.t("sea_actual_cobra_hunt")}
  {gt.t("weak_teary_penguin_fry")}
  {gt.t("sleek_topical_tadpole_tap")}
</button>
