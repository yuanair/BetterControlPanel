<script setup lang="ts">
import {onMounted} from "vue";

onMounted(() => {
  MathJax.typesetPromise();
});

</script>

<template>
  <a class="logo_outer" href="https://github.com/yuanair/BetterControlPanel/tree/MathematicalMaze"
     target="_blank">
    <img src="../assets/logo.svg" class="logo" alt="logo"/>
  </a>
  <h1>理综迷城</h1>
  <p>
    $$ y=ax^2+bx+c\quad(a\neq0) $$
    $$ E=mc^2 $$
    $$ \ce{Cl2 + H2O<=> HCl + HClO} $$
  </p>
  <button @click="$router.push('/levels')">开始冒险</button>
</template>

<style scoped>
.logo {
  height: 100%;
  width: auto;
  align-self: center;
  will-change: filter;
  transition: 0.75s;
  user-select: none;
}

.logo_outer {
  display: flex;
  align-self: center;
  height: 10em;
  width: 10em;
}

.logo:hover {
  filter: drop-shadow(0 0 5px #3a8dfa);
}
</style>