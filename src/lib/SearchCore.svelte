<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import ArrowBadge from "../svgs/ArrowBadge.svelte";

  let Query: string = "";
  let results: any[] = [];
  let suggestions: string[] = [];
  let timer: string;
  let searchState: "idle" | "searching" | "results" | "error" | "initializing" =
    "idle";

  async function Search() {
    searchState = "searching";
    try {
      const response = await invoke<Record<string, any>>("search_query", {
        query: Query,
      });
      console.log(response);
      results = response.results;
      suggestions = response.suggestions;
      timer = response.time_taken;
      console.log(timer);
      results = results.map((result) => {
        result.document = result.document
          .replace("/home/archkye/content/", "")
          .replace(".txt", "");
        return result;
      });
      searchState = "results";
    } catch (error) {
      console.log(error);
      searchState = "error";
    }
  }
</script>

<form class="flex flex-col items-center" on:submit|preventDefault={Search}>
  <div class="relative">
    <input
      type="text"
      bind:value={Query}
      class="
            text-xl
            rounded-lg bg-gray-800
            w-80 sm:w-[24rem] md:w-[32rem] lg:w-[40rem] xl:w-[48rem]
            h-16 pr-12 pl-4"
      placeholder="Search for something"
    />
    <button
      type="submit"
      class="absolute h-full right-0 top-1/2 transform -translate-y-1/2 bg-gray-800 hover:bg-gradient-to-r from-blue-500 to-purple-500 rounded-r-lg px-2"
    >
      <ArrowBadge />
    </button>
  </div>
</form>
{#if searchState === "searching"}
  <div class="typewriter mt-10">
    <div class="slide"><i></i></div>
    <div class="paper"></div>
    <div class="keyboard"></div>
  </div>
{/if}
{#if searchState === "results"}
  <article class="w-full text-white flex flex-col items-center justify-center">
    {#if timer}
      <p class="text-gray-500">Search took {timer}ms</p>
    {/if}
    {#if results.length === 0 && suggestions.length === 0}
      <p class="text-gray-500">No results!</p>
    {/if}
    <div>
      {#if suggestions.length > 0}
        Did you mean:
        {#each suggestions as suggestion, i (i)}
          <span
            >{suggestion}{#if i < suggestions.length - 1},
            {/if}</span
          >
        {/each}
      {/if}
    </div>
  </article>
  <div class="w-full max-w-3xl max-h-[500px] overflow-auto px-10">
    {#each results as result (result.document)}
      <div class="hover:bg-gray-500 cursor-default my-4">
        <h2 class="text-xl text-white">{result.document}</h2>
        <p class="text-gray-500">Score: {result.score}</p>
        <p class="text-gray-500">{result.snippet}</p>
      </div>
    {/each}
  </div>
{/if}
{#if searchState === "error"}
  <h1>Something went terrible wrong :(</h1>
{/if}
