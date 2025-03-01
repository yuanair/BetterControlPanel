<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const is_always_on_top = ref(false);
const is_window_vibrancy = ref(false);

async function lock_window() {
  is_always_on_top.value = await invoke("lock_window");
}

async function close_window() {
  await invoke("close_window");
}

async function minimize_window() {
  await invoke("minimize_window");
}

async function maximize_window() {
  await invoke("maximize_window");
}

async function window_vibrancy() {
  is_window_vibrancy.value = await invoke("window_vibrancy");
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}
</script>

<template>
  <div class="app-root">
    <div class="titlebar" data-tauri-drag-region>
      <div class="app-info">
        <img src="./assets/logo.svg" class="app-icon">
        <span>Better Control Panel</span>
      </div>
      <div class="window-controls">
        <button class="control-btn vibrancy" title="vibrancy" @click="window_vibrancy()"></button>
        <button class="control-btn pin" title="置顶" @click="lock_window()"></button>
        <button class="control-btn minimize" title="最小化" @click="minimize_window()"></button>
        <button class="control-btn maximize" title="最大化" @click="maximize_window()"></button>
        <button class="control-btn close" title="关闭" @click="close_window()"></button>
      </div>
    </div>
    <main class="container">

      <h1>Welcome to Better Control Panel</h1>

      <div class="row">
        <a href="https://github.com/yuanair/BetterControlPanel/" target="_blank">
          <img src="./assets/logo.svg" class="logo vite"/>
        </a>
      </div>

      <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
        <button type="submit">Greet</button>
      </form>
      <p>{{ greetMsg }}</p>
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

.row {
  display: flex;
  justify-content: center;
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

#greet-input {
  margin-right: 5px;
}

.titlebar {
  height: 40px;
  background: rgba(0, 0, 0, 0.3);
  display: flex;
  position: relative;
  align-items: center;
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
  right: 8px;
  gap: 8px;
}

.control-btn {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: none;
  cursor: pointer;
  transition: filter 0.2s;
}

.control-btn:hover {
  filter: brightness(1.2);
}

.minimize {
  background: #ffbd44;
}

.maximize {
  background: #00ca56;
}

.close {
  background: #ff605c;
}

.pin {
  background: #888;
  display: flex;
  justify-content: center;
  align-items: center;
  color: white;
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