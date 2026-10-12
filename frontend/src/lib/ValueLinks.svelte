<script lang="ts">
let { media } = $props();
</script>

<div class="flex items-baseline gap-3">
  <span class="w-16 font-display text-sm uppercase text-stone-500">{
    "valued"
  }</span>
  <span class="flex flex-wrap gap-x-2 font-body">
    {#each media as medium}
      <a
        href="/most_expensive/{encodeURIComponent(medium)}/50"
        class="hover:underline"
      >{medium}</a>
    {/each}
  </span>
</div>
