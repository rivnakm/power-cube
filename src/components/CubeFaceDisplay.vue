<script setup lang="ts">
import { CubeFace, FaceColor } from "../lib/models";

defineProps<{
  face: CubeFace;
}>();

function faceColorToClass(faceColor: FaceColor): string {
  switch (faceColor) {
    case FaceColor.Blue:
      return "bg-blue-500";
    case FaceColor.Red:
      return "bg-red-500";
    case FaceColor.Green:
      return "bg-green-500";
    case FaceColor.Orange:
      return "bg-orange-400";
    case FaceColor.Yellow:
      return "bg-yellow-300";
    case FaceColor.White:
      return "bg-white";
  }
}
</script>

<template>
  <div class="flex flex-row gap-1">
    <!-- eslint-disable-next-line vue/require-v-for-key -->
    <div class="flex flex-col gap-1" v-for="row in face">
      <!-- eslint-disable-next-line vue/require-v-for-key -->
      <div v-for="col in row" :class="['rounded', 'size-8', faceColorToClass(col)]"></div>
    </div>
  </div>
</template>
