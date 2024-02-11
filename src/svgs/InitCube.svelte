<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let showInput = false;
  let pAtH = "";
  let init: any;
  function handleButtonClick() {
    showInput = true;
  }

  function handleInputChange(event: any) {
    pAtH = event.target.value;
  }

  async function handleInputBlur() {
    console.log(pAtH);
    init = await invoke("directory", { path: pAtH });
    if (init) {
      console.log(init);
    }
    showInput = false;
  }
</script>

<button on:click={handleButtonClick}>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    class="stroke-[#b363ac] hover:stroke-[#f796c1]"
    width="40"
    height="40"
    viewBox="0 0 24 24"
    stroke-width="2"
    stroke="currentColor"
    fill="none"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <path stroke="none" d="M0 0h24v24H0z" fill="none" />
    <path d="M6 17.6l-2 -1.1v-2.5" />
    <path d="M4 10v-2.5l2 -1.1" />
    <path d="M10 4.1l2 -1.1l2 1.1" />
    <path d="M18 6.4l2 1.1v2.5" />
    <path d="M20 14v2.5l-2 1.12" />
    <path d="M14 19.9l-2 1.1l-2 -1.1" />
    <path d="M12 12l2 -1.1" />
    <path d="M18 8.6l2 -1.1" />
    <path d="M12 12l0 2.5" />
    <path d="M12 18.5l0 2.5" />
    <path d="M12 12l-2 -1.12" />
    <path d="M6 8.6l-2 -1.1" />
  </svg>
</button>
{#if showInput}
  <input
    class="absolute top-4 right-4 text-red-600"
    type="text"
    bind:value={pAtH}
    on:input={handleInputChange}
    on:blur={handleInputBlur}
  />
{/if}
