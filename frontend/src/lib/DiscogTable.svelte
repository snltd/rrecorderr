<script lang="ts">
import DiscogRow from "#lib/DiscogRow.svelte";
import type { Release } from "#lib/types";

let { releases, page }: { releases: Release[]; page: "artist" | "label" } =
  $props();
</script>

<table class="border-separate border-spacing-2 border border-gray-400 dark:border-gray-500">
  <thead>
    <tr>
      <th>artist</th>
      <th>title</th>
      <th>label</th>
      <th>format</th>
      <th>type</th>
      <th>catalogue</th>
      <th>released</th>
    </tr>
  </thead>
  <tbody>
    {#each releases as release}
      <DiscogRow {release} {page} />
    {/each}
  </tbody>
</table>
