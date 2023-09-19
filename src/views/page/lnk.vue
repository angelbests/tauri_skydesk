<script setup lang="ts">
import { appWindow,LogicalSize } from '@tauri-apps/api/window'
import { lnk } from '../../types';
import { reactive,ref,onMounted } from 'vue'
import rightMenu from './../../components/rightMenu.vue'
import { useRoute } from 'vue-router'
import { windowPosition } from '../../common';
import { open } from '@tauri-apps/api/shell'
import { invoke } from "@tauri-apps/api/tauri"
windowPosition()
const route = useRoute()
const data = reactive({
    lnks:[] as lnk[],
    opacity:70,
    color:'255,255,255',
    title:true,
    total:0,
});
onMounted( async ()=>{
    let dataStorage = localStorage.getItem(appWindow.label+'-data');
    if(dataStorage){
        let data1:any = JSON.parse(dataStorage);
        data.lnks.push(...data1.lnks);
        data.opacity = data1.opacity;
        data.color = data1.color;
        data.title = data1.title;
        data.total = data1.total
    }
    let row:number = parseInt(route.params.row as string)
    let col:number = parseInt(route.params.col as string)
    appWindow.setResizable(false)
    appWindow.setSize(new LogicalSize(row*70+10,col*70+10))
    data.total = row*col;
})


//#region 拖拽
const dragstart = function(event:DragEvent,item:lnk,index:number){
    event.dataTransfer?.setData(appWindow.label,index.toString());
    event.dataTransfer?.setData('lnk',JSON.stringify(item));
    console.log(item)
}

const dragover = function(event:DragEvent){
    event.preventDefault();
}

const drop = function(event:DragEvent){
    event.preventDefault();
    // 从主窗口拖拽
    if(event.dataTransfer?.getData('lnk') != ''){
        let lnkdata:any =JSON.parse(event.dataTransfer?.getData('lnk') as string)
        // 超出容量
        if(data.lnks.length==data.total){
            // alert('超出容量！');
            return
        };
        let filterdata = data.lnks.filter((item)=>{
            return item.src == lnkdata.src;
        })
        if(filterdata.length>=1){
            // alert('重复项');
            return
        }
        data.lnks.push(lnkdata);
        localStorage.setItem(appWindow.label+'-data',JSON.stringify(data))
    }
} 

const itemchange = function(event:DragEvent,index:number){
    event.preventDefault();
    if(event.dataTransfer?.getData(appWindow.label) != ''){
        let startindex:any = event.dataTransfer?.getData(appWindow.label);
        startindex = parseInt(startindex);
        let item = data.lnks[index];
        data.lnks[index] = data.lnks[startindex]
        data.lnks[startindex] = item  
        localStorage.setItem(appWindow.label+'-data',JSON.stringify(data))
    }
}

//#endregion

//#region  右键菜单代码
// 标题隐藏
const titletoggle = function(){
    data.title = !data.title
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(data))
}

// 颜色选择
const colorChange = function(){
    document.getElementById('colorBoard')?.click()
}

// 获取颜色
const colorInput = function(e:any){
    const {red,green,blue } = hexToRgba(e.srcElement.value,1);
    data.color = red+','+green+','+blue
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(data))
}
// 8进制color转10进制
function hexToRgba(hex:string, opacity:number|string) {
    let RGBA = "rgba(" + parseInt("0x" + hex.slice(1, 3)) + "," + parseInt("0x" + hex.slice(3, 5)) + "," + parseInt( "0x" + hex.slice(5, 7)) + "," + opacity + ")";
    return {
        red: parseInt("0x" + hex.slice(1, 3)),
        green: parseInt("0x" + hex.slice(3, 5)),
        blue: parseInt("0x" + hex.slice(5, 7)),
        rgba: RGBA
    }
}

// 透明度
const transparentbool = ref(false);
const rangeChange = function(e:any){
    data.opacity = e.srcElement.value
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(data))
}

// 右键菜单事件
const rightClick = function(){
    transparentbool.value = false
}
//#endregion

const openExe =async function(item:lnk){
    if(item.type=="exe"){
        await open(item.src)
    }else if(item.type=="tauricommand"){
        await invoke(item.src)
    }
} 
</script>

<template>
    <div :style="{background:'rgba('+data.color+','+data.opacity/100+')'}" draggable="true" @dragover="dragover($event)" @drop="drop($event)"  class="folder">
        <div draggable="true" @dragstart="dragstart($event,item,index)"  @dragover="dragover($event)"  @drop="itemchange($event,index)" v-for="(item,index) in data.lnks" class="menu-item" @click="openExe(item)">
            <img draggable="false" class="menu-item-img" :src="item.icon">
            <div class="menu-item-title" v-if="data.title">
                {{ item.title }}
            </div>
        </div>
    </div>
    <right-menu @right-click="rightClick">
        <img class="rightmenu-img" :src="'icon/title.png'" @click="titletoggle">
        <img class="rightmenu-img" :src="'icon/transparent.png'" @click="transparentbool = !transparentbool">
        <img class="rightmenu-img" :src="'icon/changecolor.png'" @click="colorChange" >
    </right-menu>
    <!-- 背景色选择 -->
    <input type="color" v-show="true" id="colorBoard" class="color-board" @input="colorInput">
    <!-- 透明度 -->
    <input type="range" v-show="transparentbool" @input="rangeChange" id="transparent" class="transparent" step="1" max="100" min="0">
</template>

<style scoped>
/* 透明度 */
.transparent{
    position: absolute;
    z-index: 210;
    top: 10vh;
    width: 80vw;
    margin-left: 10vw;
}
/* 色板 */
.color-board{
    position: absolute;
    bottom: 0px;
    opacity: 0;
}
/* 右键菜单 */
.rightmenu-img{
    width: 25x;
    height: 25px;
    display: flex;
}
/* 文件夹外层容器 */
.folder{
    display: flex;
    flex-wrap: wrap;
    width: 100vw;
    height: 100vh;
    border-radius: 20px;
    /* background-color: rgba(200,200,200,0.5); */
    box-sizing: border-box;
    padding: 5px;
    justify-content:flex-start;
    align-content: flex-start;
}
/* 菜单项目 */
.menu-item {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    width: 70px;
    height: 70px;
    border-radius: 5px;
    padding: 5px 5px;
    box-sizing: border-box;
}

/* .menu-item-img:hover{
    background: floralwhite;
} */

.menu-item-img{
    display: flex;
    width: 40px;
    height: 40px;
    /* pointer-events: none; */
}

.menu-item-title{
    width: 100%;
    display: flex;
    font-size:12px;
    justify-content: center;
    text-overflow: clip;
    white-space: nowrap;
    overflow: hidden;
    padding: 3px 3px;
    box-sizing: border-box;
}
</style>