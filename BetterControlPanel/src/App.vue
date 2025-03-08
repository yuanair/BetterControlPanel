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
        <span>数学迷城</span>
      </div>
      <div class="window-controls">
        <button class="control-btn settings" title="设置">
          <img class="icon" src="./assets/ant-design/ant-design--setting-outlined.svg" alt="settings"/>
        </button>
        <button class="control-btn vibrancy" title="vibrancy" @click="set_window_vibrancy()">
          <svg class="icon" xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024">
            <path :class="{ active: use_window_vibrancy }" id="svg_1"
                  d="m512,64c-247.4,0 -448,200.6 -448,448s200.6,448 448,448s448,-200.6 448,-448s-200.6,-448 -448,-448m0,820c-205.4,0 -372,-166.6 -372,-372s166.6,-372 372,-372s372,166.6 372,372s-166.6,372 -372,372"
                  fill="currentColor"/>
          </svg>
          <!--          <img v-if="use_window_vibrancy" class="icon" src="./assets/ant-design/ant-design&#45;&#45;circle-filled.svg"-->
          <!--               alt="vibrancy"/>-->
          <!--          <img v-else class="icon" src="./assets/ant-design/ant-design&#45;&#45;circle-outlined.svg" alt="vibrancy"/>-->
        </button>
        <button class="control-btn pin" title="置顶" @click="lock_window()">
          <img v-if="is_always_on_top" class="icon" src="./assets/ant-design/ant-design--pushpin-filled.svg"
               alt="pushpin"/>
          <img v-else class="icon" src="./assets/ant-design/ant-design--pushpin-outlined.svg" alt="pushpin"/>
        </button>
        <button class="control-btn minimize" title="最小化" @click="minimize_window()">
          <img class="icon" src="./assets/ant-design/ant-design--minus-outlined.svg" alt="minimize"/>
        </button>
        <button class="control-btn maximize" title="最大化" @click="maximize_window()">
          <img v-if="is_maximized" class="icon" src="./assets/ant-design/ant-design--maximize-outlined.svg"
               alt="maximize"/>
          <img v-else class="icon" src="./assets/ant-design/ant-design--border-outlined.svg" alt="maximize"/>
        </button>
        <button class="control-btn close" title="关闭" @click="close_window()">
          <img class="icon" src="./assets/ant-design/ant-design--close-outlined.svg" alt="pushpin"/>
        </button>
      </div>
    </div>
    <main class="container">

      <h1>数学迷城</h1>
      <p>LaTeX: \(E=mc^2\)</p>

      <div class="row">
        <a href="https://github.com/yuanair/BetterControlPanel/" target="_blank">
          <img src="./assets/logo.svg" class="logo vite" alt="logo"/>
        </a>
      </div>
    </main>
  </div>
</template>

<style scoped>
.logo:hover {
  filter: drop-shadow(0 0 2em #3a8dfa);
}

.app-root {
  display: flex;
  flex-direction: column;

  background: transparent;
  width: 100vw;
  height: 100vh;
}

[data-tauri-drag-region] {
  -webkit-app-region: drag;
  user-select: none;
}

.window-controls button {
  -webkit-app-region: no-drag;
}

</style>
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

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
  user-select: none;
}

.icon {
  display: flex;
  height: 16px;
  width: 16px;
  user-select: none;
  -webkit-user-select: none;
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

.titlebar {
  height: 32px;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  position: relative;
  padding: 0 12px;
  user-select: none;
}

.app-info {
  display: flex;
  align-items: center;
  gap: 8px;
  color: white;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-icon {
  width: 24px;
  height: 24px;
}

.window-controls {
  display: flex;
  position: absolute;
  right: 0;
}

.control-btn {
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 0;
  padding: 8px;
  box-shadow: none;
  background-color: transparent;
  align-items: center;
  display: flex;
  transition: all 0.2s;
}

.control-btn:hover {
  filter: brightness(1.2);
}

.minimize:hover {
  background: rgba(255, 189, 68, 0.2);
}

.maximize:hover {
  background: rgba(0, 202, 86, 0.2);
}

.close:hover {
  background: rgba(255, 96, 92, 0.2);
}

.pin:hover {
  background: rgba(136, 136, 136, 0.2);
}

.vibrancy:hover {
  background: rgba(69, 192, 255, 0.2);
}

.settings:hover {
  background: rgba(133, 133, 133, 0.2);
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  .icon {
    filter: invert(70%);
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

  .icon {
    fill: #ffffff;
  }

}

</style>