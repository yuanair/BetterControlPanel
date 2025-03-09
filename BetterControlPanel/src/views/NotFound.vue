<script setup lang="ts">

import router from "../router";
</script>

<template>
  <h1>页面丢失：404</h1>
  <p>{{ $route.path }}</p>
  <button @click="router.back()">返回</button>
  <button @click="router.push('/')">返回开始界面</button>
</template>

<style scoped>

</style>