<script setup lang="ts">
import {onMounted} from "vue";
import router from "../router";
import {FlameAnimation} from "../animation/FireAnimation";

onMounted(() => {
  MathJax.typesetPromise();
  const canvas = document.getElementById('fireCanvas') as HTMLCanvasElement;
  if (!canvas) {
    throw new Error('canvas not found');
  }
  const flame = new FlameAnimation(canvas);

// 控制接口
//   const intensityControl = document.getElementById('intensity') as HTMLInputElement;
//   intensityControl.addEventListener('input', () => {
//     flame.createParticles(Number(intensityControl.value));
//   });
//
//   document.getElementById('colorToggle')!.addEventListener('click', () => {
//     flame.toggleColorMode();
//   });
  flame.createParticles(Number(500));

// 启动动画
  flame.start();
})

const levels = [
  {id: 1, name: "数学", unlocked: true},
  {id: 2, name: "物理", unlocked: false},
  {id: 3, name: "化学", unlocked: false}
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
      {{ level.name }}
    </button>
  </div>
  <button @click="router.push('/')">返回</button>
  <canvas id="fireCanvas"></canvas>

</template>

<style scoped>
canvas {
  cursor: pointer;
}
</style>