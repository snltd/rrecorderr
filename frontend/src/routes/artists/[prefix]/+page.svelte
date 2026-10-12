<script lang="ts">
import type { PageProps } from "./$types";
let { data }: PageProps = $props();
</script>

<svelte:head>
  <title>Artists: {data.prefix}</title>
</svelte:head>

<ul>
  {#each data.artists as artist}
    <li><a href="/discog/artist/{artist.id}">{artist.name}</a></li>
  {/each}
</ul>
