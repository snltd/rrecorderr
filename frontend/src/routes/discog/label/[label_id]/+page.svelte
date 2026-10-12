<script lang="ts">
import DiscogTable from "#lib/DiscogTable.svelte";
import type { PageProps } from "./$types";

let { data }: PageProps = $props();
</script>

<DiscogTable releases={data.releases} page={"label"} />
