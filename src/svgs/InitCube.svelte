<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let showInput = false;
  let pAtH = "";
  let init: any;
  let init_state: "initializing" | "idle" = "idle";

  function handleButtonClick() {
    showInput = !showInput;
  }

  function handleInputChange(event: any) {
    pAtH = event.target.value;
  }

  async function handleInputBlur() {
    console.log(pAtH);
    init_state = "initializing";
    init = await invoke("directory", { path: pAtH });
    if (init) {
      console.log(init);
    }
    init_state = "idle";
    showInput = false;
  }
</script>

<button on:click={handleButtonClick}>
  {#if init_state === "idle"}
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
  {/if}
  {#if init_state === "initializing"}
    <div class="loader"></div>
  {/if}
</button>
{#if showInput}
  <div class="absolute right-14 mt-36 text-white">
    <input
      class="rounded-lg bg-deep-purple-gray h-9"
      type="text"
      bind:value={pAtH}
      on:input={handleInputChange}
    /><button
      on:click={handleInputBlur}
      class="absolute h-full right-0 top-1/2 transform -translate-y-1/2 bg-deep-purple-gray hover:bg-gradient-to-r from-blue-500 to-purple-500 rounded-r-lg px-2"
      ><svg
        width="24"
        height="24"
        viewBox="0 0 24 24"
        stroke-width="2"
        stroke="currentColor"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path
          d="M20 11a8.1 8.1 0 0 0 -6.986 -6.918a8.095 8.095 0 0 0 -8.019 3.918"
        />
        <path d="M4 13a8.1 8.1 0 0 0 15 3" />
        <path d="M19 16m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />
        <path d="M5 8m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />
        <path d="M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" />
      </svg>
    </button>
  </div>
{/if}
