<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <div class="h-screen w-screen bg-zinc-900 text-zinc-200">
    <div class="flex h-full w-full flex-col">
      <nav class="relative flex shrink-0 justify-center border-b border-zinc-800 text-zinc-500">
        <RouterLink
          class="m-1.5 flex flex-initial cursor-default items-center justify-center rounded-xl px-4 py-1.5 select-none hover:bg-zinc-800 hover:text-zinc-200"
          activeClass="text-zinc-200"
          to="/"
        >
          <i class="fa-solid fa-stopwatch pr-2 text-lg"></i> Timer
        </RouterLink>
      </nav>
      <div class="h-full w-full">
        <RouterView />
      </div>
    </div>
  </div>
</template>
