<script lang="ts" setup>
import { appWindow,LogicalSize } from '@tauri-apps/api/window'
import { onMounted, ref,reactive } from 'vue';
// 初始化大小
appWindow.setSize(new LogicalSize(48,48))
// 禁止窗口调整和改变
appWindow.setResizable(false);

interface historyItem {
    type:'url'|'text',
    content:string
}
const history = reactive<historyItem[]>([]);
const input = ref<string>('');
import { open } from '@tauri-apps/api/shell'
const go =async function(event:any){
    if(event.which == 13){
        let reg = /^(?:(http|https|ftp):\/\/)?((?:[\w-]+\.)+[a-z0-9]+)((?:\/[^/?#]*)+)?(\?[^#]+)?(#.+)?$/i;
        console.log(reg.exec(input.value))
        let data:historyItem = {
            type:'url',
            content:'',
        };
        let result:any= reg.exec(input.value)
        if(result == null){
            await open('https://www.baidu.com/s?ie=UTF-8&wd='+input.value);
            data.type = 'text';
        }else if(result[1] == undefined && result[2] !=undefined ){
            await open('http://'+input.value);
            data.type = 'url';
        }else if(result[1] !=undefined && result[2] != undefined){
            await open(input.value);
            data.type = 'url';
        }else{
            await open('https://www.baidu.com/s?ie=UTF-8&wd='+input.value)
            data.type = 'text';
        }
        data.content = input.value;
        let arr = history.filter((item)=>{
            return item.content == input.value
        })
        if(arr.length == 0){
            history.push(data);
        }
        localStorage.setItem(appWindow.label+'-data',JSON.stringify(history));
        showHisory.splice(0,showHisory.length);
        
        input.value = '';
        inputData();
        selectIndex.value = 999;
    }
}


const show =ref<boolean>(false)
const changesize = function(){
    document.getElementById('search-header')?.addEventListener('dblclick',function(){
        if(!show.value){
            appWindow.setSize(new LogicalSize(500,48))
            show.value = !show.value
        }else{
            showHisory.splice(0,showHisory.length);
            setTimeout(() => {
                appWindow.setSize(new LogicalSize(48,48))
            }, 100);
            show.value = !show.value
            
        }
    })
}


// 显示历史记录检索
const show2 = ref<boolean>(false);
const showHisory = reactive<historyItem[]>([]);
const inputData = function(){
    showHisory.splice(0,showHisory.length);
    const arr =  history.filter((item)=>{
        return item.content.indexOf(input.value)>=0
    });
    showHisory.push(...arr);
    if(input.value&&(arr.length>=1&&arr.length<=9)){
        show2.value = true;
        appWindow.setSize(new LogicalSize(500,30*(arr.length)+58));
    }else{
        show2.value =false;
        setTimeout(() => {
            appWindow.setSize(new LogicalSize(500,48));
        }, 100);
    }
}


// 删除所有历史记录
const clearHistory = function(){
    history.splice(0,history.length);
    localStorage.setItem(appWindow.label+'-data','[]')
    alert('清理历史记录成功！')
}

// 删除单项历史记录
const delItem = function(data:any,index:any){
    showHisory.splice(index,1)
    if(showHisory.length == 0){
        show2.value =false;
        setTimeout(()=>{
            appWindow.setSize(new LogicalSize(500,48));
        },100)
    }else{
        show2.value = true;
        setTimeout(()=>{
            appWindow.setSize(new LogicalSize(500,30*(showHisory.length)+58));
        },100)
    }
    history.filter((item,index)=>{
        if(item.content == data.content){
            history.splice(index,1);
        }
    });
}

// 方向键上下选择
const selectIndex = ref<number>(999);
document.addEventListener('keydown',function(e:any){
    if(e.which == 38){
        //up
        let dom:any = document.getElementsByClassName('historyItem');
        
        if(selectIndex.value == 999){   
            selectIndex.value = showHisory.length-1
        }else if(selectIndex.value == 0){
            dom[selectIndex.value].style.background = '';
            selectIndex.value = showHisory.length-1;
        }else{
            dom[selectIndex.value].style.background = '';
            selectIndex.value = selectIndex.value - 1;
        }
        console.log(selectIndex.value)
        dom[selectIndex.value].style.background = 'rgba(100,100,100,0.5)';
        input.value = showHisory[selectIndex.value].content
    }else if(e.which == 40){
        //down
        let dom:any = document.getElementsByClassName('historyItem');
        
        if(selectIndex.value == 999){   
            selectIndex.value = showHisory.length-1
        }else if(selectIndex.value == showHisory.length-1){
            dom[selectIndex.value].style.background = '';
            selectIndex.value = 0;
            
        }else{
            dom[selectIndex.value].style.background = '';
            selectIndex.value = selectIndex.value + 1;
        }
        console.log(selectIndex.value)
        dom[selectIndex.value].style.background = 'rgba(100,100,100,0.5)';
        input.value = showHisory[selectIndex.value].content
    }
    
})


onMounted(()=>{
    changesize();
    // 初始化历史数据
    let str = localStorage.getItem(appWindow.label+'-data');
    if(str){
        history.push(...JSON.parse(str));
    }else{
        localStorage.setItem(appWindow.label+'-data','[]')
    }
})

/////////////////////////////////////////////////////////////

import rightMenu from './../../components/rightMenu.vue'

import { windowPosition} from './../../common/index'
windowPosition();

import { hideRightMenu } from './../../common/index';
hideRightMenu();
</script>

<template>
<div class="search" :style="{borderRadius:show?'10px':'50%',width:show?'500px':'48px'}">
    <div style="display: flex;justify-content: center;align-items: center;width: 48px;width: 100vw;">
        <div class="input-header" id="search-header" data-tauri-drag-region>
            <img id="search-img" :src="'/note/search.png'" style="width: 30px;height:30px;pointer-events: none;border-radius: 50%;" />
        </div>
        <input v-show="show" class="input" v-model="input" placeholder="输入搜索内容或网站链接,回车确认搜索！"  @keypress="go($event)" @input="inputData" />
        <div v-show="show" class="input-footer"></div>
    </div>
    <div class="searchHistory" v-if="show2">
        <div v-for="(item,index) in showHisory" class="historyItem" >
            <div class="historyItem-text" @click="go({which:13})">
                {{ item.content }} 
            </div>
            <img :src="'note/del.png'" class="historyItem-del"  @click="delItem(item,index)">
        </div>
    </div>
</div>
<right-menu v-if="show" :border-radius="'10px'">
    <div class="menuitem" @click="clearHistory">
            <img :src="'icon/clear.png'" alt="" class="menuimg">
        </div>
</right-menu>
</template>


<style>
/*  */

body{
    overflow: hidden;
}
.searchHistory{
    width: 100vw;
    height: 300px;
    background-color:rgba(38,35,41,1);
    color: white;
    overflow-y: scroll;
    overflow-x: hidden;
    transition: all 0.1s linear;
}

.historyItem{
    margin-left: 60px;
    box-sizing: border-box;
    padding: 5px;
    border-top: 1px solid white;
    width: calc( 100vw - 60px - 30px );
    font-size: 14px;
    height: 30px;
    transition: all 0.1s linear;
}

.historyItem-text{
    width: 360px;
    float: left;
    margin-right: 20px;
}

.historyItem-text:hover{
    background-color:rgba(100,100,100,0.5);
    cursor: pointer;
}

.historyItem-del{
    width: 20px;
    float: left;
}
/* .historyItem-del:hover{
    background-color:rgba(100,100,100,0.5);
    cursor: pointer;
} */

::-webkit-scrollbar {
    display: none;
}


.search{
    transition: all 0.1s linear;
    width: 100vw;
    height: 100vh;
    display: flex;
    align-items: start;
    justify-content: center;
    background-color:rgba(38,35,41,1);
    overflow: hidden;
    flex-wrap: wrap;
}

.input{
    width: 428px;
    outline: none;
    border: none;
    background-color:rgb(50, 50, 50);
    height: 30px;
    border-radius: 15px;
    color: white;
    font-size: 13px;
    padding: 0px 10px;
    box-sizing: border-box;

}
.input-header {
    width: 48px;
    height: 48px;
    border: none;
    padding: none;
    margin: none;
    display: flex;
    align-items: center;
    justify-content: center;
}
.input-footer{
    width: 20px;
    height: 48px;
}

.menuitem{
    width: 30px;
    height: 30px;
    background: transparent;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 5px;
}

.menuimg{
    width: 25px;
    height: 25px;
}
</style>