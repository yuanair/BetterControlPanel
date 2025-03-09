<script setup lang="ts">

import router from "../../router";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from "@tauri-apps/api/window";

async function run_game() {
  await getCurrentWindow().minimize();
  await invoke("run_game")
}

</script>

<template>
  <button @click="router.back()">返回</button>
  <button @click="run_game()">启动</button>
</template>

<style scoped>

</style>