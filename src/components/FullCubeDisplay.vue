<script setup lang="ts">
import { Cube, FaceName } from "../lib/models";
import CubeFaceDisplay from "./CubeFaceDisplay.vue";

defineProps<{ cube: Cube }>();
</script>

<template>
  <div class="grid grid-cols-[1fr_max-content_1fr]">
    <div></div>
    <div class="grid grid-cols-4 gap-1.5">
      <CubeFaceDisplay class="col-start-2" :face="cube.faces[FaceName.Up]" />
      <CubeFaceDisplay class="col-start-1" :face="cube.faces[FaceName.Left]" />
      <CubeFaceDisplay :face="cube.faces[FaceName.Front]" />
      <CubeFaceDisplay :face="cube.faces[FaceName.Right]" />
      <CubeFaceDisplay :face="cube.faces[FaceName.Back]" />
      <CubeFaceDisplay class="col-start-2" :face="cube.faces[FaceName.Down]" />
    </div>
    <div></div>
  </div>
</template>
