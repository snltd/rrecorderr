<script lang="ts">
import DiscogRow from "#lib/DiscogRow.svelte";
import type { Release } from "#lib/types";

let { releases, page, currentId }: {
  releases: Release[];
  page: "artist" | "label";
  currentId: number;
} = $props();

let sortField = $state<keyof Release>("released");
let sortDir = $state<"asc" | "desc">("asc");

function setSort(field: keyof Release) {
  if (sortField === field) {
    sortDir = sortDir === "asc" ? "desc" : "asc";
  } else {
    sortField = field;
    sortDir = "asc";
  }
}

let sorted = $derived(
  [...releases].sort((a, b) => {
    const av = a[sortField] ?? "";
    const bv = b[sortField] ?? "";
    let cmp: number;

    if (typeof av === "number" && typeof bv === "number") {
      cmp = av - bv;
    } else {
      cmp = av > bv ? 1 : av < bv ? -1 : 0;
    }

    return sortDir === "asc" ? cmp : -cmp;
  }),
);
</script>

<table class="[&_td]:py-1.5 [&_th]:py-1.5 w-full table-auto text-sm border-collapse">
  <thead>
    <tr class="border-b-2">
      <th class="text-left pr-4" onclick={() => setSort("artists")}>by</th>
      <th class="text-left px-4" onclick={() => setSort("title")}>title</th>
      <th class="text-left px-4" onclick={() => setSort("label_name")}>
        label
      </th>
      <th class="text-left px-4" onclick={() => setSort("medium")}>on</th>
      <th class="text-left px-4" onclick={() => setSort("release_type")}>is</th>
      <th class="text-left px-4" onclick={() => setSort("cat_no")}>cat</th>
      <th class="text-left px-4" onclick={() => setSort("released")}>year</th>
      <th class="text-left pl-4" onclick={() => setSort("value")}>value</th>
    </tr>
  </thead>
  <tbody>
    {#each sorted as release}
      <DiscogRow {release} {page} {currentId} />
    {/each}
  </tbody>
</table>
