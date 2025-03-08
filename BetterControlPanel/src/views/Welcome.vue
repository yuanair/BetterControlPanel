<script setup lang="ts">
import "../utils/mathjax";
import "mathjax/es5/tex-svg";
import {onMounted} from "vue";

onMounted(() => {
  MathJax.typesetPromise();
})

</script>

<template>
  <h1>理综迷城</h1>
  <p>LaTeX: \(y=ax^2+bx+c\quad(a\neq0)\)</p>

  <div class="row">
    <a href="https://github.com/yuanair/BetterControlPanel/tree/MathematicalMaze" target="_blank">
      <img src="../assets/logo.svg" class="logo vite" alt="logo"/>
    </a>
    <router-link to="/levels">
      <button class="button">开始</button>
    </router-link>
  </div>
</template>

<style scoped>
.logo:hover {
  filter: drop-shadow(0 0 2em #3a8dfa);
}
</style>