<script setup lang="ts">
import { onBeforeMount } from "vue";
import { RouterLink, RouterView, useRouter } from "vue-router";
import { checkJava } from "./lib/invoke_wrapper";
import { useErrorStore } from "./stores/error";

const router = useRouter();
const errorStore = useErrorStore();

onBeforeMount(async () => {
  const err = await checkJava();
  if (err !== null) {
    errorStore.message = "Java installation check failed!";
    errorStore.helpMessage = "Ensure Java 11+ is installed and available in PATH";
    errorStore.description = err;
    router.replace("/error");
  }
});
</script>

<template>
  <div class="h-screen w-screen bg-zinc-900 text-zinc-200">
    <div class="grid h-full w-full grid-rows-[3rem_1fr]">
      <nav class="relative flex shrink-0 justify-center border-b border-zinc-800 text-zinc-500">
        <RouterLink
          class="m-1.5 flex flex-initial cursor-default items-center justify-center rounded-xl px-4 py-1.5 select-none hover:bg-zinc-800 hover:text-zinc-200"
          activeClass="text-zinc-200"
          to="/"
        >
          <i class="fa-solid fa-stopwatch pr-2 text-lg"></i> Timer
        </RouterLink>
      </nav>
      <div class="min-h-0 w-full">
        <RouterView />
      </div>
    </div>
  </div>
</template>
