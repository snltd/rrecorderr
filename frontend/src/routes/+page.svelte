<script lang="ts">
let { data } = $props();
</script>

<ul>
  {#each data.prefixes as prefix}
    <li><a href="/artists/{prefix}">{prefix}</a></li>
  {/each}
</ul>
