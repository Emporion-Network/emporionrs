export function trapFocus(node: HTMLElement) {
  const previous = document.activeElement as HTMLElement;

  function focusable() {
    return Array.from(
      node.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
      ),
    ) as HTMLElement[];
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key !== "Tab") return;

    const current = document.activeElement;

    const elements = focusable();
    const first = elements.at(0) as HTMLElement;
    const last = elements.at(-1) as HTMLElement;

    if (event.shiftKey && current === first) {
      last?.focus();
      event.preventDefault();
    }

    if (!event.shiftKey && current === last) {
      first.focus();
      event.preventDefault();
    }
  }

  $effect(() => {
    focusable()[0]?.focus();
    node.addEventListener("keydown", handleKeydown);

    return () => {
      node.removeEventListener("keydown", handleKeydown);
      previous?.focus();
    };
  });
}

// export type DataAttribute = `${string}=${string}`;
// export function data(node:HTMLElement, value?:DataAttribute){
//   if(value){
//     node.setAttribute(...value.split('=') as [string, string])
//   }
// }