<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const scramble = ref("");

async function getScramble() {
  scramble.value = await invoke("get_scramble");
}
</script>

<template>
  <button @click="() => getScramble()">Get scramble</button>
  <p>{{ scramble }}</p>
</template>
