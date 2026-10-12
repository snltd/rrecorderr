<script lang="ts">
import ValuedTable from "#lib/ValuedTable.svelte";
import type { PageProps } from "./$types";

let { data }: PageProps = $props();
</script>

<ValuedTable releases={data.releases} } />
