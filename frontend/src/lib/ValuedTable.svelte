<script lang="ts">
import type { ValuedRelease } from "#lib/types";
import ValuedRow from "#lib/ValuedRow.svelte";

let { releases }: ValuedRelease[] = $props();
</script>

<table class="border-separate border-spacing-2 border border-gray-400 dark:border-gray-500">
  <thead>
    <tr>
      <th>artist</th>
      <th>title</th>
      <th>label</th>
      <th>type</th>
      <th>catalogue</th>
      <th>Estimated</th>
      <th>UK</th>
      <th>World</th>
    </tr>
  </thead>
  <tbody>
    {#each releases as release}
      <ValuedRow {release} />
    {/each}
  </tbody>
</table>
