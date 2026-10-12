<script lang="ts">
import "../app.css";
import AlphaLinks from "#lib/AlphaLinks.svelte";
import ValueLinks from "#lib/ValueLinks.svelte";

let { data, children } = $props();
</script>

<div class="min-h-screen bg-stone-50 font-body text-stone-900">
  <header class="flex items-start gap-4 border-b border-stone-300 px-4 py-3">
    <a href="/" class="font-display text-2xl leading-none" aria-label="Home"
    >&#8962;</a>
    <nav class="flex flex-col gap-1">
      <AlphaLinks
        label="artist"
        base="/artists"
        letters={data.artistInitials}
      />
      <AlphaLinks label="label" base="/labels" letters={data.labelInitials} />
      <ValueLinks label="valued" media={data.media} />
    </nav>
  </header>

  <main class="p-4">
    {@render children()}
  </main>
</div>
