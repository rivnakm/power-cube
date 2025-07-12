<script setup lang="ts">
import { useErrorStore } from "../stores/error";

const errorStore = useErrorStore();
</script>

<template>
  <div class="flex h-full flex-col place-content-center text-center">
    <h2 class="text-2xl text-red-400">{{ errorStore.message }}</h2>
    <h3>{{ errorStore.helpMessage }}</h3>
    <p class="pt-4 font-mono">{{ errorStore.description }}</p>
  </div>
</template>
