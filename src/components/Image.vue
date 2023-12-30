<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from 'vue'
import { PhysicalSize, LogicalPosition } from '@tauri-apps/api/window'
import { listen } from '@tauri-apps/api/event'

const imgPath = ref("/nothing.png");
let width = 0, height = 0, scaleFactor = 1, lastScaleFactor = 1;

function change_image(path){
  if(path != undefined) 
    imgPath.value = path;
  var image = new Image();
  image.src = imgPath.value;
  image.onload = () => {
    width = image.width;
    height = image.height;
    resize();
  }
}

async function resize() {
  let win = window.__TAURI__.window.getCurrent();
  let size = new PhysicalSize(Math.round(width * scaleFactor), Math.round(height * scaleFactor));
  await win.setSize(size).catch(console.error);
}

const onLoaded = async () => {
  window.oncontextmenu = function (e) {
    e.preventDefault()
  }
  change_image();
}

let isChanging = false;
let deltaY = 0;

const handleWheel = async (e) => {
  deltaY += e.deltaY;
  if (isChanging) return;

  isChanging = true;

  let delta = deltaY * 0.01 / 50
  scaleFactor = Math.max(0.2, scaleFactor - delta);
  let win = window.__TAURI__.window.getCurrent();
  let screenX = e.screenX;
  let screenY = e.screenY;
  let left = e.clientX;
  let top = e.clientY;
  let position = new LogicalPosition(Math.round(screenX - left / lastScaleFactor * scaleFactor), Math.round(screenY - top / lastScaleFactor * scaleFactor));
  await win.setPosition(position).catch(console.error)
  await resize();

  lastScaleFactor = scaleFactor

  isChanging = false;
  deltaY = 0;
}

const unlisten = await listen('setImage', (e) => {
  // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
  // event.payload is the payload object]
  change_image(e.payload.message);
})
</script>
<template>
  <img data-tauri-drag-region ref="el" :src="imgPath" @load="onLoaded" @mousewheel="handleWheel" />
</template>
<style scoped>
img {
  margin: 0;
  border: 0;
  width: 100%;
  height: 100%;
  object-fit: cover;
}
</style>