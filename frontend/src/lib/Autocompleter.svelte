

<script lang="ts">
  import {ngrams, findMap} from "./utils";
  let {
    completions = $bindable(),
    value = $bindable(),
    placeholder
  }: {
    completions:string[],
    value?:string,
    placeholder:string
  } = $props();


  let completionNgrams = $derived(completions.map(e=>[e, ngrams(e)] as [string, string[]]))
  let suggestion = $derived.by(()=>{
    if (!value) return "";
    let toMatch= value.toLocaleLowerCase();
    let completion = findMap(completionNgrams, ([text, ngrams]) => {
      let ngram = ngrams.find(ngram => {
        return toMatch.endsWith(ngram) && /\B/u.test(toMatch[toMatch.length - ngram.length - 1])
      });
      return ngram ? text.slice(ngram?.length) : false;
    }) || "";

    return value + completion;
  });
  let r:DOMRectReadOnly = $state(new DOMRectReadOnly());
  let textarea:HTMLTextAreaElement ;

  const oninput = ()=>{
    textarea.style.height =  'auto';
    textarea.style.height = `${textarea.scrollHeight}px`;
  }

  const autocomplete = (e:KeyboardEvent)=>{
    if(["Tab"].includes(e.key)){
        e.preventDefault()
        if(suggestion){
          value = suggestion;
        }
    }
  }
</script>
<div class="autocompleter">
    <textarea bind:value={value} class="native" bind:contentRect={r} bind:this={textarea} {oninput} {placeholder} onkeydown={autocomplete}></textarea>
    <div style="--w:{r.width}px;--h:{r.height}px">{suggestion}</div>
</div>

<style lang="scss">
    .autocompleter{
        position: relative;
        overflow: auto;
        width: 100%;
        height: 100%;
        textarea{
            padding: 0;
            margin:0;
            width: 100%;
            height: 100%;
            outline: none;
            border: none;
            font-size: 1rem;
            resize: none;
            background-color: transparent;
            color: var(--neutral-12);
        }
        &:focus-within{
            div{
                opacity: 0.5;
            }
        }
        div{
            position: absolute;
            top:0;
            left: 0;
            font-size: 1rem;
            pointer-events: none;
            white-space: preserve wrap;
            word-wrap:break-word;
            width: var(--w);
            height: var(--h);
            max-width: var(--w);
            max-height: var(--h);
            opacity: 0;
        }

    }
</style>
