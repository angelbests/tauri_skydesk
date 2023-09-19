<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { ref,onMounted,reactive } from 'vue'
import { appWindow,PhysicalPosition } from '@tauri-apps/api/window'
import { open } from '@tauri-apps/api/shell'
import { lnk } from './../../types'

appWindow.hide();
appWindow.setResizable(false)
appWindow.setAlwaysOnTop(true)
const show = ref(false)
const position = reactive({
    x:0,
    y:0,
    factor:1
})

const un1 = ref()
const un2 = ref()
const lnks = reactive<lnk[]>([
    {
        icon:'note/notepad.png',
        src:'C:/Windows/notepad.exe',
        title:'记事本',
        type:"exe"
    },
    {
        icon:'note/resource-manager.png',
        src:'C:/Windows/explorer.exe',
        title:'资源管理器',
        type:"exe"
    },
    {
        icon:'note/regedit.png',
        src:'C:/Windows/regedit.exe',
        title:'注册表',
        type:"exe"
    },
    {
        icon:'note/calc.png',
        src:'C:/Windows/system32/calc.exe',
        title:'计算器',
        type:"exe"
    },
]);

onMounted(async ()=>{
    let roulettelnksstr = localStorage.getItem('roulette');
    if(roulettelnksstr){
        lnks.splice(0,lnks.length)
        lnks.push(...JSON.parse(roulettelnksstr));
    }

    // 鼠标滚轮点击事件
    
    position.factor =await appWindow.scaleFactor()
    un1.value = await listen('wheel-click',(event:any) => {
        console.log(event)
        if(event.payload.message == 'ButtonPress(Middle)'){
            appWindow.setFocus()
            if(!show.value){
                appWindow.setPosition(new PhysicalPosition(position.x-100*position.factor,position.y-100*position.factor))
            }
            appWindow.show()
            show.value = true
        }else if(event.payload.message == 'ButtonRelease(Middle)'){
            appWindow.hide()
            show.value = false
            
        }
    })

    un2.value =await listen('mouse-move', (event:any) => {
        let str = event.payload.message.substring(11,event.payload.message.length-1)
        let arr = str.split(',')
        let x = arr[0].split(":")[1]
        let y = arr[1].split(":")[1]
        position.x = parseInt(x);
        position.y = parseInt(y);
    });


     window.addEventListener('storage',function(e:StorageEvent){
        if(e.key == 'roulette'){
            lnks.splice(0,lnks.length)
            lnks.push(...JSON.parse(e.newValue as string))
        }
     })
})


const jd1time = ref();
const jd2time = ref();
const jd3time = ref();
const jd4time = ref();
const jd1 =async function(){
    appWindow.hide();
    clearTimeout(jd1time.value);
    jd1time.value = setTimeout(async function(){
        if(lnks[0].type=='exe'){
            await open(lnks[0].src)
        }else if(lnks[0].type=='tauricommand'){
            await invoke(lnks[0].src)
        }
    },20)
}

const jd2 =async function(){
    appWindow.hide();
    clearTimeout(jd2time.value);
    jd2time.value = setTimeout(async function(){
        if(lnks[1].type=='exe'){
            await open(lnks[1].src)
        }else if(lnks[1].type=='tauricommand'){
            await invoke(lnks[1].src)
        }
    },20)
}

const jd3 =async function(){
    appWindow.hide();
    clearTimeout(jd3time.value);
    jd3time.value = setTimeout(async function(){
        if(lnks[2].type=='exe'){
            await open(lnks[2].src)
        }else if(lnks[2].type=='tauricommand'){
            await invoke(lnks[2].src)
        }
    },20)
}

const jd4 =async function(){
    appWindow.hide();
    clearTimeout(jd4time.value);
    jd4time.value = setTimeout(async function(){
        if(lnks[3].type=='exe'){
            await open(lnks[3].src)
        }else if(lnks[3].type=='tauricommand'){
            await invoke(lnks[3].src)
        }
    },20)
}

import rightMenu from '../../components/rightMenu.vue'
</script>

<template>
    <right-menu :border-radius="'50%'"></right-menu>
    <svg width="200" height="200" style="transform: rotateZ('45deg');" >
            <circle id='center' cx='100px' cy='100px' r='100px' fill="none"></circle>
            <image :href="lnks[3].icon" x="35" y="115" height="50px" width="50px"/>   
            <image :href="lnks[0].icon" x="35" y="35" height="50px" width="50px"/>   
            <image :href="lnks[2].icon" x="115" y="115" height="50px" width="50px"/>   
            <image :href="lnks[1].icon" x="115" y="35" height="50px" width="50px"/>   
            <path d="M 0 100
                    A 100 100 0 0 0 100 200
                    L 100 100 Z" fill="rgba(123,123,123,0.1)" class="jd1" @mouseover="jd4"/>
            <path d="M 100 200
                    A 100 100 0 0 0 200 100
                    L 100 100 Z" fill="rgba(123,123,123,0.1)" class="jd2" @mouseover="jd3"/>
            <path d="M 200 100
                    A 100 100 0 0 0 100 0
                    L 100 100 Z" fill="rgba(123,123,123,0.1)" class="jd3" @mouseover="jd2"/>
            <path d="M 100 0
                    A 100 100 0 0 0 0 100
                    L 100 100"   fill="rgba(123,123,123,0.1)" class="jd4" @mouseover="jd1"/>
                     
            <circle cx='100px' cy='100px' r='20px' fill="white" fill-opacity="1"></circle>
    </svg>
</template>

<style>
#app{
    width: 100vw;
    height: 100vh;
}
html,body{
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    /* background-color: white; */
}
.jd1:hover{
    fill:rgba(123,123,123,0.5)
}
.jd2:hover{
    fill:rgba(123,123,123,0.5)
}
.jd3:hover{
    fill:rgba(123,123,123,0.5)
}
.jd4:hover{
    fill:rgba(123,123,123,0.5)
}
</style>