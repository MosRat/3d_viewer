<script setup>
import {getCurrent} from "@tauri-apps/api/window";
import {ref} from "vue";

const props = defineProps(['logo'])
const appWin = getCurrent()

const isMaximized = ref(false)
// const title = ref(props.title ?? "Hello Tauri")
const logo = ref(props.logo ?? "/integrate-light (1).svg")


</script>

<template>
  <div data-tauri-drag-region class="title_bar">
    <div data-tauri-drag-region class="title_bar-title_box">
      <!--      <div class="title_bar-title_logo">-->
      <!--        <img :src="logo" alt="">-->
      <!--      </div>-->
      <img class="title_bar-title_logo" :src="logo" alt="">
      <p class="title_bar-title_text">
        <slot>Title</slot>
      </p>
    </div>
    <div class="title_bar-buttons-area">
      <div class="title_bar-button title_bar-close" id="title_bar-close" @click="appWin.close()">
        <img
            src="/ant-design_close-outlined.svg"
            alt="close"
        />
      </div>

      <div v-if="!isMaximized" class="title_bar-button" id="title_bar-maximize"
           @click="()=>{appWin.maximize();isMaximized=true}">
        <img
            src="/fluent_maximize-16-regular.svg"
            alt="maximize"
        />
      </div>
      <div v-if="isMaximized" class="title_bar-button" id="title_bar-maximize"
           @click="()=>{appWin.unmaximize();isMaximized=false}">
        <img
            src="/fluent_window-multiple-16-regular.svg"
            alt="unmaximize"
        />
      </div>

      <div class="title_bar-button" id="title_bar-minimize" @click="appWin.minimize()">
        <img
            src="/fluent_minimize-16-regular.svg"
            alt="minimize"
            style="width: 16px;height: 16px"
        />
      </div>

    </div>
  </div>
</template>

<style scoped>
.title_bar {
  margin: 0;
  padding: 0;
  height: 5vh;
  width: 100%;
  //text-align: center;
  line-height: 2vh;
  font-weight: normal;
  font-size: 0.9rem;
  //display: flex;
  //flex-direction: row;
  //justify-content: right;
  //align-content: center;

}

.title_bar-title_box {
  float: left;
  width: 70%;
  display: flex;
  align-content: center;

}

.title_bar-title_logo {
  text-align: center;
  //height: 20px;
  //width: 20px;
  //padding: 6px 12px;
  //text-align: center;
  //display: flex;
  //justify-content: center;
  //align-content: center;
  padding: calc((5vh - 3vh) / 2);
  width: 3vh;
  height: 3vh;
  will-change: filter;
  transition: 0.75s;
}


.title_bar-title_logo:hover {
  filter: drop-shadow(0 0 0.8em #2489db);
}


.title_bar-title_text {
  padding-left: 0.5vw;
  padding-block: 1.5vh;
  margin: 0;
  //width: 30px;
  //height: 1.25em;
  //padding: 0.75em 1em;
  //height: 16px;
  //width: 96px;
  //text-align: left;
  //line-height: 28px;
  //font-family: sans-serif;
  //font-size: 12px;
  //font-weight: 200;
  //border: #0f0f0f dashed 2px;
}

.title_bar-button {
  //width: 16px;
  height: 16px;
  padding: calc((5vh - 16px) / 2) 1vw;
  float: right;
  //text-align: center;
  //display: flex;
  //justify-content: center;
  //align-content: center;
}

.title_bar-button:hover {
  background-color: #737373;
}

.title_bar-close:hover {
  background-color: #737373 !important;
}


.title_bar-buttons-area {
  float: left;
  width: 30%;
}


</style>