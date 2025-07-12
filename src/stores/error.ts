import { defineStore } from "pinia";
import { ref } from "vue";

export const useErrorStore = defineStore("error", () => {
  const message = ref("");
  const helpMessage = ref("");
  const description = ref("");

  return { message, helpMessage, description };
});
