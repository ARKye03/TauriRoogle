<script lang="ts">
  import SearchResults from "./lib/SearchCore.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  let showInput = false;
  let pAtH = "";

  function handleButtonClick() {
    showInput = true;
  }

  function handleInputChange(event: any) {
    pAtH = event.target.value;
  }

  function handleInputBlur() {
    console.log(pAtH);
    invoke("directory", { path: pAtH });
    showInput = false;
  }
</script>

<main>
  <section class="">
    <button class="absolute top-4 right-4" on:click={handleButtonClick}
      >A</button
    >
    {#if showInput}
      <input
        class="absolute top-4 right-4 text-red-600"
        type="text"
        bind:value={pAtH}
        on:input={handleInputChange}
        on:blur={handleInputBlur}
      />
    {/if}
    <div class="flex flex-col justify-between items-center">
      <h1
        class="my-16 text-3xl text-white hover:text-blue-500 transition-colors duration-200 cursor-default"
      >
        Welcome to tauri
      </h1>
      <SearchResults />
    </div>
  </section>
</main>
