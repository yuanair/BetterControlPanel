<script setup lang="ts">
import {onMounted} from "vue";

onMounted(() => {
  MathJax.typesetPromise();
});

</script>

<template>
  <h1>理综迷城</h1>
  <p>
    $$ y=ax^2+bx+c\quad(a\neq0) $$
    $$ E=mc^2 $$
    $$ \ce{Cl2 + H2O<=> HCl + HClO} $$
  </p>

  <div class="row">
    <a href="https://github.com/yuanair/BetterControlPanel/tree/MathematicalMaze" target="_blank">
      <img src="../assets/logo.svg" class="logo vite" alt="logo"/>
    </a>
  </div>
  <button @click="$router.push('/levels')">开始冒险</button>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
  user-select: none;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #3a8dfa);
}
</style>