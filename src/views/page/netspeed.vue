<script lang="ts" setup>
import { reactive,onMounted } from 'vue'
import { listen} from '@tauri-apps/api/event'
import rightMenu from '../../components/rightMenu.vue';
import { appWindow,LogicalSize } from '@tauri-apps/api/window'
import { windowPosition,forbidSelect } from '../../common';
appWindow.setSize(new LogicalSize(85,50))
appWindow.setResizable(false)
appWindow.setAlwaysOnTop(true)
windowPosition();
forbidSelect();
const speedshow = reactive<{
    received:number,
    unit1:'MB/s'|'KB/s'
    transmitted:number
    unit2:'MB/s'|'KB/s'
}>({
    received:0,
    transmitted:0,
    unit1:'KB/s',
    unit2:'KB/s'
})

onMounted(async ()=>{
    getspeed();
})

const getspeed =async function(){
    await listen('netspeed',function(event:any){
        let  speedstr= event.payload.message.split('||||');
        let received = 0;
        let transmitted = 0;
        for(let i = 0;i<speedstr.length-1;i++){
            let str = speedstr[i].split(':')[1];
            received = received + Number(str.split('/')[0]);
            transmitted = transmitted + Number(str.split('/')[1]);
        }
        console.log(received,transmitted);
        speedshow.received = Math.floor(received/1024)>=1000?Number((received/1024/1024).toFixed(1)):Number((received/1024).toFixed(1))
        speedshow.transmitted = Math.floor(transmitted/1000)>=1000?Number((transmitted/1024/1024).toFixed(1)):Number((transmitted/1024).toFixed(1))
        speedshow.unit1 = Math.floor(received/1024)>=1000?'MB/s':'KB/s'
        speedshow.unit2 = Math.floor(transmitted/1024)>=1000?'MB/s':'KB/s'
    })
}
</script>
<template>
<div class="speed" data-tauri-drag-region>
    <div style="width: 100vw;display: flex;">
        <img :src="'/note/up.png'" style="width: 13px;pointer-events: none;">
        <div class="transmitted" data-tauri-drag-region>
            {{ speedshow.transmitted }}
        </div>
        <div style="display: flex;width: 25px;" data-tauri-drag-region>
            {{ speedshow.unit2 }}
        </div>
    </div>
    <div style="width: 100vw;display: flex;">
        <img :src="'/note/down.png'" style="width: 13px;pointer-events: none;">
        <div class="received" data-tauri-drag-region>
            {{ speedshow.received }}
        </div>
        <div style="display: flex;width: 25px;" data-tauri-drag-region>
            {{ speedshow.unit1 }} 
        </div>
    </div>
</div>
<right-menu :border-radius="'5px'"></right-menu>
</template>
<style>
html,body{
    overflow: hidden;
    cursor: default;
}
.speed{
    width: 100vw;
    height: 100vh;
    background: rgba(20,20,20,0.3);
    flex-wrap: wrap;
    font-size: 10px;
    border-radius: 5px;
    padding: 10px 5px;
    box-sizing: border-box;
    display: flex;
    align-items: center;
    color: yellow;
}

.received{
    text-align: right;
    height: 10px;
    width: 35px;
}

.transmitted{
    text-align: right;
    width: 35px;
    height: 10px;
}
</style>