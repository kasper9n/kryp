<script lang="ts">
  import TextInput from '$lib/TextInput.svelte'

  let year = String(new Date().getFullYear())
</script>

<div class="flex h-32 items-center p-5">
  <p class="mr-4">Year</p>
  <TextInput value={year} />
</div>
