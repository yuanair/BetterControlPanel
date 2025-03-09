<script setup lang="ts">

import {invoke} from "@tauri-apps/api/core";
import {onMounted, ref} from "vue";
import router from "../../router";
import {onUpdated} from "@vue/runtime-core";

onMounted(() => {
  MathJax.typesetPromise();
});
onUpdated(() => {
  MathJax.typesetPromise();
});


const current_quadratic_equation = ref<string>("");
const user_input_x1 = ref("");
const user_input_x2 = ref("");
const user_input_is_right = ref([false, ""]);

async function create_quadratic_equation() {
  const result = await invoke<string>("create_quadratic_equation");
  console.log(result);
  current_quadratic_equation.value = result;
}

async function is_right() {
  user_input_is_right.value = await invoke<[boolean, string]>("is_right", {
    x1: user_input_x1.value,
    x2: user_input_x2.value
  });
}

async function help() {
  const right = await invoke("get_right")
  alert(`答案为：${right}`);
}

create_quadratic_equation();

</script>

<template>
  <button @click="router.back()">返回</button>
  <button @click="create_quadratic_equation()">新题目</button>
  <button @click="help()">帮助</button>
  <p>
    $$
    {{ current_quadratic_equation }}
    $$
  </p>
  <input v-model="user_input_x1" placeholder="输入结果x1" type="text" @input="is_right()"/>
  <input v-model="user_input_x2" placeholder="输入结果x2" type="text" @input="is_right()"/>
  <p>{{ user_input_is_right[1] }}<br></p>
  <p>由于涉及到虚数计算，倒推公式，时间有限，只能做到部分功能。</p>
</template>

<style scoped>

</style>