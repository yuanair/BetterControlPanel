<script setup lang="ts">
import "../utils/mathjax";
import "mathjax/es5/tex-svg";
import {onMounted} from "vue";

onMounted(() => {
  MathJax.typesetPromise();
})

const levels = [
  {id: 1, unlocked: true},
  {id: 2, unlocked: false},
  {id: 3, unlocked: false}
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
        @click="0/*selectLevel(level)*/"
        :disabled="!level.unlocked"
    >
      关卡 {{ level.id }}
    </button>
  </div>
  <router-link to="/">
    <button class="back-button">返回</button>
  </router-link>
</template>

<style scoped>

</style>