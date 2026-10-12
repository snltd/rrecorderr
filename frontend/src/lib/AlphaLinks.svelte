<script lang="ts">
let { label, base, letters }: {
  label: string;
  base: string;
  letters: string[];
} = $props();
</script>

<div class="flex items-baseline gap-3">
  <span class="w-16 font-display text-sm uppercase text-stone-500">{
    label
  }</span>
  <span class="flex flex-wrap gap-x-2 font-body">
    {#each letters as letter}
      <a href="{base}/{encodeURIComponent(letter)}" class="hover:underline">{
        letter
      }</a>
    {/each}
  </span>
</div>
