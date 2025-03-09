/* eslint-disable */
<script setup lang="ts">
import {onMounted} from "vue";
import router from "../router";

onMounted(() => {
  MathJax.typesetPromise();
})

const levels = [
  {id: "Math", name: "数学", unlocked: true},
  {id: "Physics", name: "物理", unlocked: true},
  {id: "Chemistry", name: "化学", unlocked: true}
];

</script>

<template>
  <h2>选择关卡</h2>
  <div class="levels-grid">
    <button
        v-for="level in levels"
        :key="level.id"
        class="level-button"
        :class="{ locked: !level.unlocked }"
        @click="router.push(`/levels/${level.id}`)"
        :disabled="!level.unlocked"
    >
      {{ level.name }}
    </button>
  </div>
  <button @click="router.push('/')">返回</button>
</template>

<style scoped>
</style>