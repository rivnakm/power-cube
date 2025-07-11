import { globalIgnores } from "eslint/config";
import { defineConfigWithVueTs, vueTsConfigs } from "@vue/eslint-config-typescript";
import pluginVue from "eslint-plugin-vue";
import skipFormatting from "@vue/eslint-config-prettier/skip-formatting";

export default defineConfigWithVueTs(
  {
    name: "app/files-to-lint",
    files: ["**/*.{ts,mts,vue}"],
  },

  globalIgnores(["**/dist/**", "**/lib/**", "**/target/**", "**/vite-env.d.ts"]),

  pluginVue.configs["flat/essential"],
  vueTsConfigs.recommended,

  skipFormatting,
);
