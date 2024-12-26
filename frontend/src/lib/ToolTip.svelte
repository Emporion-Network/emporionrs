<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        children,
        content,
        openTimout = 0,
        closeTimeout = 0,
    }: {
        children: Snippet;
        content: Snippet;
        openTimout?: number;
        closeTimeout?: number;
    } = $props();

    let visible = $state(false);
    let pos = $state("");
    let beforeOpen:ReturnType<typeof setTimeout>;
    let beforeClose:ReturnType<typeof setTimeout>;

    let el: HTMLElement;
    const open = ()=>{
        const rect = el.getBoundingClientRect();
        const parentRect = el.parentElement!.getBoundingClientRect();
        let x = "left";
        let y = "bottom";
        if (rect.width + parentRect.x > window.innerWidth) {
            x = "right";
        }
        if (rect.height + parentRect.y > window.innerHeight) {
            y = "top";
        }
        pos = `${y}-${x}`;
        visible = true;
    }
    const show = () => {
        beforeOpen = setTimeout(open, openTimout);
        clearTimeout(beforeClose);
    };
    const hide = () => {
        beforeClose = setTimeout(() => {
            visible = false;
            pos = "";
        }, closeTimeout);
        clearTimeout(beforeOpen);
    };
</script>

<div class="tooltip" class:visible>
    <div
        class="wpr"
        onmouseenter={show}
        onmouseleave={hide}
        onfocus={show}
        onblur={hide}
        role="tooltip"
    >
        {@render children()}
    </div>
    <div class="content {pos}" bind:this={el}>
        {@render content()}
    </div>
</div>

<style lang="scss">
    .tooltip {
        position: relative;
        z-index: 1;
        .wpr {
            display: contents;
        }
        .content {
            position: fixed;
            opacity: 0;
            pointer-events: none;
            max-width: 300px;
            width: max-content;
            background-color: var(--neutral-3);
            padding: 0.5rem;
            border: 1px solid var(--neutral-6);
            border-radius: 3px;
            &.bottom-left {
                top: 100%;
                left: 0%;
            }
            &.bottom-right {
                top: 100%;
                right: 0%;
            }
            &.top-left {
                bottom: 100%;
                left: 0%;
            }
            &.top-right {
                bottom: 100%;
                right: 0%;
            }
            &.bottom-center {
                top: 100%;
                left: 50%;
                transform: translateX(-50%);
            }
            &.top-center {
                bottom: 100%;
                left: 50%;
                transform: translateX(-50%);
            }
        }
        &.visible {
            .content {
                opacity: 1;
                pointer-events: all;
                position: absolute;
            }
        }
    }
</style>
