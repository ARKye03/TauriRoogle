<script lang="ts">
  import SearchResults from "./lib/SearchCore.svelte";
  import SideBar from "./components/SideBar.svelte";

  
</script>

<main>
  <div
    class="absolute inset-0 -z-10 h-full w-full items-center px-5 py-24 [background:radial-gradient(125%_125%_at_50%_10%,#000_40%,#63e_100%)]"
  ></div>
  <section class="">
    <SideBar />
    
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
