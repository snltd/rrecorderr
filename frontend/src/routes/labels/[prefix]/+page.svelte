<script lang="ts">
interface Artist {
  id: number;
  name: string;
}

let { data } = $props();
</script>

<ul>
  {#each data.labels as label}
    <li><a href="/discog/label/{label.id}">{label.name}</a></li>
  {/each}
</ul>
