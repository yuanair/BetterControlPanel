<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const is_always_on_top = ref(false)

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

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", {name: name.value});
}
</script>

<template>
  <div class="titlebar" data-tauri-drag-region>
    <div class="app-info">
      <img src="./assets/vue.svg" class="app-icon">
      <span>Better Control Panel</span>
    </div>
    <div class="window-controls">
      <button class="control-btn pin" title="置顶" @click="lock_window()">📌</button>
      <button class="control-btn minimize" title="最小化" @click="minimize_window()"></button>
      <button class="control-btn maximize" title="最大化" @click="maximize_window()"></button>
      <button class="control-btn close" title="关闭" @click="close_window()"></button>
    </div>
  </div>
  <main class="container">

    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo"/>
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo"/>
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo"/>
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
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

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
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
  background: #2d2d2d;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 12px;
  user-select: none;
}

.app-info {
  display: flex;
  align-items: center;
  gap: 8px;
  color: white;
}

.app-icon {
  width: 24px;
  height: 24px;
}

.window-controls {
  display: flex;
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