<script lang="ts" setup>
import { ref,onMounted } from 'vue'
import { appWindow } from '@tauri-apps/api/window'

export interface Props {
    borderRadius?: string
}
const props = withDefaults(defineProps<Props>(), {
    borderRadius: '20px',
})

const emit = defineEmits(['rightClick']);

const show = ref(false);
onMounted(()=>{
    document.addEventListener("contextmenu",function(event:any){
        event.preventDefault();
        emit('rightClick');
        show.value = !show.value
    });
    console.log(props)
})

const top = ref(false);
const topToggle = function(){
    top.value = !top.value
    appWindow.setAlwaysOnTop(top.value);
    let dataStorage = localStorage.getItem(appWindow.label)
    if(dataStorage){
        let data = JSON.parse(dataStorage);
        data.alwaysOnTop = top.value
        localStorage.setItem(appWindow.label,JSON.stringify(data));
    }
}

// 关闭并清理数据
const close = function(){
    localStorage.removeItem(appWindow.label),
    localStorage.removeItem(appWindow.label+'-data')
    let windowsStr = localStorage.getItem('windows')
    if(windowsStr){
        let windows = JSON.parse(windowsStr);
        windows.splice(windows.indexOf(appWindow.label),1)
        localStorage.setItem('windows',JSON.stringify(windows))
    }
    appWindow.close();
}

</script>

<template>
    <div v-show="show" :style="{'borderRadius':borderRadius}" class="rightmenu" data-tauri-drag-region>
        <slot></slot>
        <img draggable="false" class="rightmenu-img" :src="top?'icon/topping.png':'icon/down.png'" @click="topToggle">
        <img draggable="false" class="rightmenu-img" :src="'icon/close.png'" @click="close">
    </div>
</template>

<style>
.rightmenu{
    top: 0px;
    position: absolute;
    z-index: 200;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-content:center;
    width: 100vw;
    height: 100vh;
    background: rgba(200, 200, 200, 0.2);
}
.rightmenu-img{
    width: 25x;
    height: 25px;
    display: flex;
}
</style>