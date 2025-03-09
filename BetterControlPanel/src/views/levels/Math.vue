<script setup lang="ts">

import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";
import {onMounted} from "vue";
import router from "../../router";

onMounted(() => {
  MathJax.typesetPromise();
});

interface QuadraticEquation {
  a: number;
  b: number;
  c: number;
}

const current_quadratic_equation = ref<QuadraticEquation>({a: 0, b: 0, c: 0});
const user_input_x1 = ref(0);
const user_input_x2 = ref(0);
const user_input_is_right = ref(false);

async function create_quadratic_equation() {
  const result = await invoke<QuadraticEquation>("create_quadratic_equation", {x1: [0, 0], x2: [1, 0], a: 1});
  console.log(result);
  current_quadratic_equation.value = result;
}

async function is_right(): Promise<boolean> {
  const right = await invoke<boolean>("get_right", {equation: current_quadratic_equation.value})
  const result = await invoke<boolean>("is_right", {
    equation: current_quadratic_equation.value,
    x1: [user_input_x1.value, 0],
    x2: [user_input_x2.value, 0]
  });
  console.log(right, result);
  return result;
}

create_quadratic_equation();

</script>

<template>
  <button @click="router.back()">返回</button>
  <h1>因为涉及到虚数计算，倒推公式，所以时间有限，只能做到部分功能。</h1>
  <p>$$ {{ current_quadratic_equation.a }}x^2 + {{ current_quadratic_equation.b }}x +
    {{ current_quadratic_equation.c }} = 0 $$</p>
  <input v-model="user_input_x1" placeholder="输入结果x1" type="number" @input="is_right()"/>
  <input v-model="user_input_x2" placeholder="输入结果x2" type="number" @input="is_right()"/>
  <p v-if="user_input_is_right">结果：{{ user_input_is_right ? "正确" : "错误" }}</p>
</template>

<style scoped>

</style>