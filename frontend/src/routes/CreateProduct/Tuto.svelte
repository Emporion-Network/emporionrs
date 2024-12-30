<script lang="ts">
    import Tuto from "../../lib/Tuto.svelte";
    import { getTranslator } from "../../stores/translate.svelte";
    let {
        ondone,
    }: {
        ondone: () => void;
    } = $props();
    let t = getTranslator();
</script>

<Tuto
    {ondone}
    elementSelectors={[
        "language",
        "product_title",
        "product_description",
        "translate",
        "product_collection",
        "add_attribute",
    ] as const}
    step={0}
>
    {#snippet help(v)}
        {#if v === "language"}
            <h3>{t.t("odd_hour_felix_wish")}</h3>
            <p>
               {t.t("mealy_lazy_wallaby_boil")}
            </p>
        {/if}
        {#if v === "product_title"}
            <h3>{t.t("nice_brave_poodle_explore")}</h3>
            <p>
                {t.t("noisy_ideal_hyena_affirm")}
            </p>
        {/if}
        {#if v === "product_description"}
            <h3>{t.t("late_short_samuel_shine")}</h3>
            <p>
                {t.t("curly_slimy_lobster_evoke")}
            </p>
        {/if}
        {#if v === "translate"}
            <h3>{t.t("front_still_shark_fall")}</h3>
            <p>
                {t.t("spicy_mad_warbler_attend")}
            </p>
        {/if}
        {#if v === "product_collection"}
            <h3>{t.t("ornate_kind_hare_pull")}</h3>
            <p>
               {t.t("clear_front_jackdaw_skip")}
            </p>
        {/if}
        {#if v === "add_attribute"}
            <h3>{t.t("stout_sea_polecat_favor")}</h3>
            <p>
               {t.t("tangy_active_anaconda_glow")}
            </p>
        {/if}
    {/snippet}
</Tuto>
