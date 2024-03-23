<script lang="ts">
  import Register from "$lib/components/Register.svelte";
  import "../app.css";

  export let data;
</script>

<div
  class="parent flex flex-col h-full justify-center items-center bg-green-300 bg-opacity-60"
>
  <Register ws={data.ws} />
</div>
