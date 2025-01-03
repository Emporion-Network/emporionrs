<script lang="ts">
    import { onMount, type ComponentProps, type SvelteComponent } from "svelte";
    import Tuto from "../../lib/Tuto.svelte";
    import { getTranslator } from "../../stores/translate.svelte";
    import { getTutoRegistry } from "./tutoStore.svelte";
    import { typeStr, wait } from "../../lib/utils";
    let {
        ondone,
    }: {
        ondone: () => void;
    } = $props();
    let t = getTranslator();
    let registry = getTutoRegistry();
    let targetEl:HTMLElement = $state()!;
    let disabledNext = $state(false);
    let point = $state((e:HTMLElement)=>{});

    onMount(()=>{
        const e = registry["lang_selector"] as SvelteComponent;
        targetEl = e.element;
    })
    const steps = [
        {
            stepName: "lang_selector",
            in(){
                const e = registry["lang_selector"] as SvelteComponent;
                targetEl = e.element;
            }
        },
        {
            stepName: "collection_name",
            async in() {
                let t = (registry["collection_name"] as SvelteComponent);
                point(t.element);
                await wait(200);
                targetEl = t.element;
                await typeStr("iPhone 16", (s) => {
                    t.actions.setValue(s);
                });
            },
        },
        {
            stepName: "attribute_selector",
            async in() {
                let v = (registry["attribute_selector"] as SvelteComponent);
                await wait(200);
                await point(v.element);
                targetEl = v.element;
                await wait(500);
                await point(targetEl);
                v.actions.open();
                await wait(500);
                let b = v.element.querySelector(".options>button");
                await point(b);
                v.actions.select("buttons");
                b = v.element.querySelector(".add_attribute");
                await point(b);
                v.actions.add();
                await wait(500);
                await point(v.element);
                v.actions.open();
                await wait(500);
                b = v.element.querySelectorAll(".options>button")[1];
                await point(b);
                v.actions.select("select");
                b = v.element.querySelector(".add_attribute");
                await point(b);
                v.actions.add();
            },
        },
        {
            stepName:"attribute_0",
            async in(){
                const e = registry["attribute_0_elem"] as SvelteComponent;
                const t = registry["attribute_0"] as SvelteComponent;
                await point(t.element.querySelector('input'));
                targetEl = t.element;
                await wait(200);
                await typeStr("Color", (v)=>{
                    e.actions.setValue(v)
                })
            },
        },
        {
            stepName:"attribute_1",
            async in(){
                const e = registry["attribute_1_elem"] as SvelteComponent;
                const t = registry["attribute_1"] as SvelteComponent;
                await point(t.element.querySelector('input'));
                targetEl = t.element;
                await wait(200);
                await typeStr("Size", (v)=>{
                    e.actions.setValue(v)
                })
            },
        },
        {
            stepName:"gallery",
            async in(){
                const e = registry["add_product"] as HTMLElement;
                //@ts-ignore
                targetEl = null;
                await point(e);
                e.click()
                await wait(400);
                const g = registry["gallery"] as SvelteComponent;
                targetEl = g.element;
                await wait(600);
                await point(targetEl);
                await wait(300);
                g.actions.addImage("https://www.keychron.com/cdn/shop/products/Keychron-Q6-QMK-VIA-custom-mechanical-keyboard-full-size-layout-full-aluminum-black-frame-B-knob-for-Mac-Windows-with-hot-swappable-Gateron-G-Pro-switch-red-Q6-M1Z.jpg?v=1659514285&width=1214")
                await point(targetEl.querySelector(".carousel>div")!);
                await wait(300);
                g.actions.addImage("https://www.keychron.com/cdn/shop/products/Keychron-Q6-QMK-VIA-custom-mechanical-keyboard-full-size-layout-full-aluminum-black-frame-B-knob-for-Mac-Windows-with-hot-swappable-Gateron-G-Pro-switch-red-Q6-M1Z.jpg?v=1659514285&width=1214")
            }
        },
        {
            stepName:"translate_image",
            async in(){
                targetEl = registry["translate_image_0"] as HTMLElement;
            }
        },
        {
            stepName:"product_name",
            async in(){
                await point(targetEl);
                targetEl.click();
                let b = (registry["translate_image_1"] as HTMLElement);
                await point(b);
                b.click();
                const e = (registry["product_name"] as SvelteComponent);
                targetEl = e.element;
                await point(targetEl);
                await typeStr("Product name", (v)=>{
                    e.actions.setValue(v)
                })
            }
        },
        {
            stepName:"translate_product",
            async in(){
                targetEl = targetEl.querySelector('button')!;
                await point(targetEl);
                targetEl.click();
            }
        },
        {
            stepName:"fill_missing",
            async in(){
                let e = (registry["product_description"] as SvelteComponent);
                targetEl = e.element;
                await point(targetEl);
                await typeStr("Product description", (v)=>{
                    e.actions.setValue(v)
                })
                e = (registry["product_attribute_0"] as SvelteComponent);
                targetEl = e.element;
                await point(targetEl);
                await typeStr("Red", (v)=>{
                    e.actions.setValue(v)
                })
                e = (registry["product_attribute_1"] as SvelteComponent);
                targetEl = e.element;
                await point(targetEl);
                await typeStr("250GB", (v)=>{
                    e.actions.setValue(v)
                })
            }
        },
        {
            stepName:"close_product",
            async in(){
                
                let b = (registry["close_product"] as HTMLElement);
                await point(b);
                point(null as any);
                //@ts-ignore
                targetEl = null;
                b.click()
                await wait(500)
                b = (registry["products"] as HTMLElement);
                targetEl = b;
                await point((registry["add_product"] as HTMLElement));
                
            }
        }
    ] as ComponentProps<typeof Tuto>["steps"];
</script>

<Tuto {steps} step={0} {ondone} bind:targetEl {disabledNext} bind:point>
    {#snippet help(step)}
        {@const v = step.stepName}
        {#if v === "lang_selector"}
            <h3>{t.t("odd_hour_felix_wish")}</h3>
            <p>
                {t.t("mealy_lazy_wallaby_boil")}
            </p>
        {/if}
        {#if v === "collection_name"}
            <h3>{t.t("ornate_kind_hare_pull")}</h3>
            <p>
                {t.t("clear_front_jackdaw_skip")}
            </p>
        {/if}
        {#if v === "attribute_selector"}
            <h3>{t.t("stout_sea_polecat_favor")}</h3>
            <p>
                {t.t("tangy_active_anaconda_glow")}
            </p>
        {/if}
        {#if v === "attribute"}
            <h3>Attribute</h3>
            <p>
                Define a characteristic of the product, such as size, color,
                material, or style. Attributes help create product variations
                and allow users to choose specific options when viewing the
                product.
            </p>
        {/if}
        {#if v === "gallery"}
            <h3>{t.t("alert_tired_rooster_drum")}</h3>
            <p>
                {t.t("arable_neat_donkey_inspire")}
            </p>
        {/if}
        {#if v === "translate_image"}
            <h3>{t.t("alive_elegant_barbel_urge")}</h3>
            <p>
                {t.t("icy_tough_falcon_pray")}
            </p>
        {/if}
    {/snippet}
</Tuto>

<!-- <Tuto
    {ondone}
    step={0}
>
    {#snippet help(v)}
        {#if v === "gallery"}
            <h3>{t.t("alert_tired_rooster_drum")}</h3>
            <p>
                {t.t("arable_neat_donkey_inspire")}
            </p>
        {/if}
        {#if v === "gallery_translator"}
          
        {/if}
        {#if v === "attribute"}
            <h3>Attribute</h3>
            <p>
                Define a characteristic of the product, such as size, color,
                material, or style. Attributes help create product variations
                and allow users to choose specific options when viewing the
                product.
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
       
        {#if v === "add_attribute"}
            <h3>{t.t("stout_sea_polecat_favor")}</h3>
            <p>
                {t.t("tangy_active_anaconda_glow")}
            </p>
        {/if}
    {/snippet}
</Tuto> -->
