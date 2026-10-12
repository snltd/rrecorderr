<script lang="ts">
let { release } = $props();
</script>

<tr>
  <td>
    {#if release.artists.length === 1}
      <a href="/discog/artist/{release.artists[0].id}">{
        release.artists[0].name
      }</a>
    {:else}
      Various
    {/if}
  </td>

  <td>{release.title}</td>
  <td>
    <a href="/discog/label/{release.label_id}">{release.label_name}</a>
  </td>
  <td>{release.release_type}</td>
  <td>{release.cat_no}</td>
  <td>£{release.value_estimated}</td>
  <td>£{release.value_cheapest_uk}</td>
  <td>£{release.value_cheapest_world}</td>
</tr>
