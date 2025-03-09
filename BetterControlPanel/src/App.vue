<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {getCurrentWindow} from '@tauri-apps/api/window'


const app_window = getCurrentWindow();
const is_always_on_top = ref(true);
const is_maximized = ref(true);
const use_window_vibrancy = ref(false);

async function lock_window() {
  is_always_on_top.value = !is_always_on_top.value;
  await app_window.setAlwaysOnTop(is_always_on_top.value);
}

async function close_window(this: Window) {
  await app_window.close();
}

async function minimize_window() {
  await app_window.minimize();
}

async function maximize_window() {
  is_maximized.value = !is_maximized.value;
  if (is_maximized.value) {
    await app_window.maximize();
  } else {
    await app_window.unmaximize();
  }
}

async function set_window_vibrancy() {
  use_window_vibrancy.value = !use_window_vibrancy.value;
  if (use_window_vibrancy.value) {
    await invoke("apply_window_vibrancy", {app_window: app_window});
  } else {
    await invoke("clear_window_vibrancy", {app_window: app_window});
  }
}

// 初始化
lock_window();
maximize_window();
set_window_vibrancy();

</script>

<template>
  <div class="app-root">
    <div class="titlebar" data-tauri-drag-region>
      <div class="app-info">
        <img src="./assets/logo.svg" class="app-icon" alt="logo">
        <span>理综迷城</span>
      </div>
      <div class="window-controls">
        <button class="control-btn" id="settings" title="设置">
          <img class="icon" src="./assets/ant-design/ant-design--setting-outlined.svg" alt="settings"/>
        </button>
        <button class="control-btn" id="vibrancy" title="风格" @click="set_window_vibrancy()">
          <img v-if="use_window_vibrancy" class="icon" src="./assets/ant-design/ant-design--circle-filled.svg"
               alt="vibrancy"/>
          <img v-else class="icon" src="./assets/ant-design/ant-design--circle-outlined.svg" alt="vibrancy"/>
        </button>
        <button class="control-btn" id="pin" title="置顶" @click="lock_window()">
          <img v-if="is_always_on_top" class="icon" src="./assets/ant-design/ant-design--pushpin-filled.svg"
               alt="pushpin"/>
          <img v-else class="icon" src="./assets/ant-design/ant-design--pushpin-outlined.svg" alt="pushpin"/>
        </button>
        <button class="control-btn " id="minimize" title="最小化" @click="minimize_window()">
          <img class="icon" src="./assets/ant-design/ant-design--minus-outlined.svg" alt="minimize"/>
        </button>
        <button class="control-btn" id="maximize" title="最大化" @click="maximize_window()">
          <img v-if="is_maximized" class="icon" src="./assets/ant-design/ant-design--maximize-outlined.svg"
               alt="maximize"/>
          <img v-else class="icon" src="./assets/ant-design/ant-design--border-outlined.svg" alt="maximize"/>
        </button>
        <button class="control-btn" id="close" title="关闭" @click="close_window()">
          <img class="icon" src="./assets/ant-design/ant-design--close-outlined.svg" alt="pushpin"/>
        </button>
      </div>
    </div>
    <div class="container">
      <div class="background-formulas" ref="formulaContainer"></div>
      <router-view id="container-view"></router-view>
    </div>

  </div>
</template>

<style scoped>


.app-root {
  display: flex;
  flex-direction: column;

  background: transparent;
  width: 100vw;
  height: 100vh;
}
</style>
<style scoped src="./styles/window-controls.css"></style>
<style scoped src="./styles/icon.css"></style>
<style>

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  overflow: hidden;
  box-sizing: border-box;

  color: #0f0f0f;
  background: transparent !important;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;


}

::-webkit-scrollbar {
  width: 8px;
  height: 8px;
  color: transparent;
}

::-webkit-scrollbar-track {
  background-color: rgba(0, 0, 0, 0);
}

::-webkit-scrollbar-thumb {
  background-color: rgba(1, 1, 1, 0.2);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgba(1, 1, 1, 0.3);
}

::-webkit-scrollbar-corner {
  background-color: transparent;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex: 1;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  overflow: auto;
}


a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
  text-shadow: #000000 5px 0 5px;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 0.6em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
  color: #3a8dfa;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }

}

</style>
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

.background-formulas {
  position: fixed;
  width: 100vw; /* 扩大容器范围 */
  height: 100vh;
  overflow: hidden;
  z-index: -1;
  pointer-events: none;
}
</style>

<style>
.track {
  position: absolute;
  width: 100%;
  transform: rotate(45deg);
  transform-origin: center; /* 确保旋转中心正确 */
  margin: 10vh 0; /* 增加垂直间距 */
}

.formula {
  white-space: nowrap;
  font-size: 1.2rem;
  color: rgba(0, 0, 0, 0.15);
  padding: 15px 30px;
  display: inline-block;
  position: relative;
}

.formula:nth-child(odd) {
  z-index: 1;
  filter: brightness(0.9); /* 创建深度层次 */
  animation: formulaSlide 10s linear infinite;
}

.formula:nth-child(even) {
  z-index: 0;
  animation: formulaSlide 10s linear infinite reverse;
  opacity: 0.8; /* 降低偶数层透明度 */
}

@keyframes formulaSlide {
  0% {
    transform: translateX(-200%);
  }
  100% {
    transform: translateX(200%);
  }
}

</style>
<script lang="ts">
export default {
  mounted() {
    const container = this.$refs.formulaContainer as HTMLElement;
    const formulas = [
      '\\sin(x) + \\cos(y) = 1',
      '\\int_{0}^{\\infty} e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}',
      '\\nabla \\cdot \\mathbf{E} = \\frac{\\rho}{\\epsilon_0}',
      'F = ma',
      '\\sum_{n=1}^{\\infty} \\frac{1}{n^2} = \\frac{\\pi^2}{6}'
    ];
    const formulasHtml = formulas.map<string>(formula => {
      return MathJax.tex2chtml(formula, {em: 12, ex: 6, display: false}).outerHTML
    });
    for (let i = 0; i < 5; i++) {
      let track = document.createElement('div');
      track.classList.add('track');
      track.id = `track${i}`;
      track.style.top = `${(i % 2) * 200}px`;
      for (let j = 0; j < 5; j++) {
        const elem = document.createElement('span');
        elem.classList.add('formula');
        track.appendChild(elem);
        elem.innerHTML = formulasHtml[j % 5];
      }
      container.appendChild(track);

    }
  }
};
</script>