<script lang="ts">
import type { Release } from "#lib/types";

let { release, page }: { release: Release; page: "artist" | "label" } =
  $props();
</script>

<tr>
  <td>
    {#if release.artists.length === 1}
      {#if page === "label"}
        <a href="/discog/artist/{release.artists[0].id}">{
          release.artists[0].name
        }</a>
      {:else}
        {release.artists[0].name}
      {/if}
    {:else}
      Various
    {/if}
  </td>

  <td>{release.title}</td>
  <td>
    {#if page === "artist"}
      <a href="/discog/label/{release.label_id}">{release.label_name}</a>
    {:else}
      {release.label_name}
    {/if}
  </td>
  <td>{release.medium}</td>
  <td>{release.release_type}</td>
  <td>{release.cat_no}</td>
  <td>{release.released}</td>
  <td>£{release.value}</td>
</tr>
