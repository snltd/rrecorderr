<script lang="ts">
import type { Release } from "#lib/types";

let { release, page, currentId }: {
  release: Release;
  page: "artist" | "label";
  currentId: number;
} = $props();
</script>

<tr class="align-top border-b-1 border-gray-400">
  <td class="pr-4">
    {#if release.artists.length === 1}
      {#if page === "label"}
        <a href="/discog/artist/{release.artists[0].id}">{
          release.artists[0].name
        }</a>
      {:else}
        {release.artists[0].name}
      {/if}
    {:else if release.artists.length === 3}
      {#each release.artists as artist, i}
        {#if artist.id !== currentId && artist.name !== "Various"}
          <a href="/discog/artist/{artist.id}">{artist.name}</a>
        {:else if artist.name !== "Various"}
          {artist.name}
        {/if}{i === 0 ? " / " : ""}
      {/each}
    {:else}
      Various
    {/if}
  </td>

  <td class="px-4"><i>{release.title}</i></td>
  <td class="px-4">
    {#if page === "artist"}
      <a href="/discog/label/{release.label_id}">{release.label_name}</a>
    {:else}
      {release.label_name}
    {/if}
  </td>
  <td class="px-3">{release.medium.replace(" inch vinyl", "\"")}</td>
  <td class="px-3">{release.release_type}</td>
  <td class="px-3">{release.cat_no.replace(" ", "").toLowerCase()}</td>
  <td class="px-3">{release.released}</td>
  <td class="pl-4">£{release.value.toFixed(2)}</td>
</tr>
