<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import TimerDisplay from "../components/TimerDisplay.vue";

const scramble = ref("");

async function newScramble() {
  scramble.value = await invoke("get_scramble");
}
</script>

<template>
  <TimerDisplay @reset="newScramble" />
  <p>{{ scramble }}</p>
</template>
