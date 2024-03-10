<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import Head from "./components/Head.vue";

import Virtual from "./components/Virtual.vue";
</script>

<template>
  <!--  <div class="container">-->
  <!--    <h1>Welcome to Tauri!</h1>-->

  <!--    <div class="row">-->
  <!--      <a href="https://vitejs.dev" target="_blank">-->
  <!--        <img src="/vite.svg" class="logo vite" alt="Vite logo" />-->
  <!--      </a>-->
  <!--      <a href="https://tauri.app" target="_blank">-->
  <!--        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />-->
  <!--      </a>-->
  <!--      <a href="https://vuejs.org/" target="_blank">-->
  <!--        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />-->
  <!--      </a>-->
  <!--    </div>-->

  <!--    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>-->

  <!--    <p>-->
  <!--      Recommended IDE setup:-->
  <!--      <a href="https://code.visualstudio.com/" target="_blank">VS Code</a>-->
  <!--      +-->
  <!--      <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>-->
  <!--      +-->
  <!--      <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank"-->
  <!--        >Tauri</a-->
  <!--      >-->
  <!--      +-->
  <!--      <a href="https://github.com/rust-lang/rust-analyzer" target="_blank"-->
  <!--        >rust-analyzer</a-->
  <!--      >-->
  <!--    </p>-->

  <!--    <Greet />-->
  <!--    <el-button >pri</el-button>-->

  <!--    <div>-->
  <!--      <el-button :icon="Search" circle />-->
  <!--      <el-button type="primary" :icon="Edit" circle />-->
  <!--      <el-button type="success" :icon="Check" circle />-->
  <!--      <el-button type="info" :icon="Message" circle />-->
  <!--      <el-button type="warning" :icon="Star" circle />-->
  <!--      <el-button type="danger" :icon="Delete" circle />-->
  <!--    </div>-->

  <!--    <div class="mb-4">-->
  <!--      <el-button>Default</el-button>-->
  <!--      <el-button type="primary">Primary</el-button>-->
  <!--      <el-button type="success">Success</el-button>-->
  <!--      <el-button type="info">Info</el-button>-->
  <!--      <el-button type="warning">Warning</el-button>-->
  <!--      <el-button type="danger">Danger</el-button>-->
  <!--    </div>-->

  <!--  </div>-->

  <Head>MyViewer</Head>
  <Virtual></Virtual>

</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}


</style>
