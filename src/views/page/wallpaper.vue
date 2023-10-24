<script lang="ts" setup>
import { LogicalSize, appWindow } from '@tauri-apps/api/window';
import rightMenu from '../../components/rightMenu.vue';
import { onMounted, reactive, ref, watch } from 'vue';
import { wallpaperType } from './../../types';
import { hideRightMenu} from './../../common/index'
import { invoke,convertFileSrc } from '@tauri-apps/api/tauri';
import { fetch,ResponseType } from '@tauri-apps/api/http';
import { writeBinaryFile,BaseDirectory, createDir,copyFile,exists} from '@tauri-apps/api/fs';
import { pictureDir,appDataDir,basename } from '@tauri-apps/api/path';
import { Loading } from '@opentiny/vue'
hideRightMenu();
appWindow.setSize(new LogicalSize(1000,600));
appWindow.setResizable(false);
const wallpapers = reactive<wallpaperType[]>([]);
const wallpaper = reactive({
    downloadsrc:'',
    httpsrc:'./wallpaper/a.jpg'
})
const num  = ref(0);

onMounted(async ()=>{ 
    // 创建用户图片文件夹
    await createDir('skydesk',{dir:BaseDirectory.Picture,recursive:true});
    await createDir('wallpaper',{dir:BaseDirectory.AppData,recursive:true});
    await getwallpaper();
    await savewallpaper(wallpapers[num.value].path || '');
    // 滚轮事件
    wheeltoggle();
})

const getwallpaper =async function(){
    let href = 'https://wallhaven.cc/api/v1/search?purity=100';
    // categories  sorting  order  resolutions  ratios
    if(categories_value.value != ''){
        href = href +'&categories='+categories_value.value
    }
    if(sorting_value.value != ''){
        href = href + '&sorting='+sorting_value.value
    }
    if(order_value.value != ''){
        href = href +'&order='+order_value.value
    }
    if(ratio_value.value != ''){
        href = href + '&ratios='+ratio_value.value
    }
    if(resolutions_value.value != ''){
        href = href + '&resolutions='+resolutions_value.value
    }
    page.value = page.value + 1;
    href = href + '&page='+page.value;
    const response:any = await fetch( href , {
        method: 'GET',
        timeout: 10000,
    });
    num.value = wallpapers.length;
    if(response.data.data == undefined){
        appWindow.close();
    }
    wallpapers.push(...response.data.data);
    console.log(wallpapers);
}

const savewallpaper =async function(path:string){
    let load = Loading.service({
        text:'',
        background: 'rgba(0, 0, 0, 0.3)',
        size:'large'
    })
    let filename =await basename(path);
    wallpaper.downloadsrc  =await appDataDir() +'wallpaper\\'+ filename;
    let bool = await exists(wallpaper.downloadsrc);
    if(!bool){
        let response:any = await fetch(path, {
            method: 'GET',
            timeout: 1000,
            headers:{
                ContentType: 'application/octet-stream',
            },
            responseType:ResponseType.Binary
        });
        await writeBinaryFile('wallpaper\\'+filename,new Uint8Array(response.data),{dir:BaseDirectory.AppData});
    }
    wallpaper.httpsrc = convertFileSrc(wallpaper.downloadsrc);
    load.close();
    return true;
}

//#region 切换
const pre =async function(){
    if(num.value > 0){
        num.value = num.value - 1;
        await savewallpaper(wallpapers[num.value].path || '');
    }
}

const autoplay =async function(){
    if(num.value == wallpapers.length - 1){
        await getwallpaper();
        await savewallpaper(wallpapers[num.value].path || '');
    }else{
        num.value = num.value + 1;
        await savewallpaper(wallpapers[num.value].path || '');
    }
}
//#endregion

//#region 设置壁纸
const setwallpaper =async function(){
    let path = wallpaper.downloadsrc;
    let filename =await basename(path);
    let picturesrc =await pictureDir() +'skydesk\\'+ filename;
    await copyFile(path,picturesrc,{dir:BaseDirectory.Picture})
    await invoke("setwallpaper",{src:picturesrc});
}
//#endregion

//#region 滚轮
const wheeltoggle = function(){
    document.getElementById('wallpaper')?.addEventListener('wheel',function(e:WheelEvent){
        console.log(e);
        if(e.deltaY>0){
            autoplay();
        }else if(e.deltaY<0){
            pre();
        }
    })
}
//#endregion


//#region 设置
const page = ref(1);
const categories_value = ref('');
const sorting_value =ref('');
const order_value = ref('');
const ratio_value = ref('');
const resolutions_value = ref('');
const setting_show = ref(false);
const setting =async function(){
    setting_show.value = false; 
    await getwallpaper();
    num.value = num.value + 1;
    await savewallpaper(wallpapers[num.value].path || '');
}

const categories = reactive([
    {text:'动漫',value:'010'},
    {text:'常规',value:'100'},
    {text:'人物',value:'111'},
    {text:'空',value:''}
]);

// 排序
const sorting = reactive([
    {text:'相关性',value:'relevance'},
    {text:'随机',value:'random'},
    {text:'浏览量',value:'views'},
    {text:'下载量',value:'favorites'},
    {text:'流行',value:'toplist'},
    {text:'日期',value:'date added'},
    {text:'热度',value:'hot'},
    {text:'空',value:''}
]);

// 正序 倒序
const order = reactive([
    {text:'正序',value:'asc'},
    {text:'倒序',value:'desc'},
    {text:'空',value:''}
])

// 分辨率
const ratio = reactive([
    {
        text:'5:4',
        value:'5:4',
        resolutions:[
            {text:'1280x1024',value:'1280x1024'},
            {text:'1600x1280',value:'1600x1280'},
            {text:'1920x1536',value:'1920x1536'},
            {text:'2560x2048',value:'2560x2048'},
            {text:'3840x3072',value:'3840x3072'},
            {text:'空',value:''}
        ]
    },
    {
        text:'4:3',
        value:'4:3',
        resolutions:[
            {text:'1280x960',value:'1280x960'},
            {text:'1600x1200',value:'1600x1200'},
            {text:'1920x1440',value:'1920x1440'},
            {text:'2560x1920',value:'2560x1920'},
            {text:'3840x2880',value:'3840x2880'},
            {text:'空',value:''}
        ]
    },
    {
        text:'16:10',
        value:'16:10',
        resolutions:[
            {text:'1280x800',value:'1280x800'},
            {text:'1600x1000',value:'1600x1000'},
            {text:'1920x1200',value:'1920x1200'},
            {text:'2560x1600',value:'2560x1600'},
            {text:'3840x2400',value:'3840x2400'},
            {text:'空',value:''}
        ]
    },
    {
        text:'16:9',
        value:'16:9',
        resolutions:[
            {text:'1280x720',value:'1280x720'},
            {text:'1600x900',value:'1600x900'},
            {text:'1920x1080',value:'1920x1080'},
            {text:'2560x1440',value:'2560x1440'},
            {text:'3840x2160',value:'3840x2160'},
            {text:'空',value:''}
        ]
    },
    {
        text:'Ultrawide',
        value:'Ultrawide',
        resolutions:[
            {text:'2560x1080',value:'2560x1080'},
            {text:'3840x1440',value:'3840x1440'},
            {text:'3840x1600',value:'3840x1600'},
            {text:'空',value:''}
        ]
    },
    {
        text:'空',
        value:'',
        resolutions:[
            {text:'空',value:''}
        ] 
    }
])

const resolutions = reactive<{
    text:string,
    value:string
}[]>([
    {text:'空',value:''}
]);

watch(ratio_value,(newValue:string)=>{
    ratio.filter((item,index,arr)=>{
        if(item.value == newValue){
            resolutions.length = 0
;            resolutions.push(...arr[index].resolutions);
             resolutions_value.value = resolutions[0].value   
        }
    })
})
//#endregion
</script> 

<template>
<div class="wallpaper" data-tauri-drag-region>
    <img id="wallpaper" class="img" :src="wallpaper.httpsrc" data-tauri-drag-region /> 
</div>
<div class="pre" @click="pre">
    <img class="pimg" :src="'icon/pre.png'">
</div>
<div class="next" @click="autoplay">
    <img class="pimg" :src="'icon/next.png'">
</div>
<div class="btnbar">
    <tiny-button class="btn" type="primary"  @click="setwallpaper" round>设为壁纸</tiny-button>
    <tiny-button class="btn" type="primary"  @click="setting_show = true" round>设置</tiny-button>
</div>

<div class="setting" v-show="setting_show">
    <div class="groupbtn">
        <div class="groupbtntitle">
            图片分类
        </div>
        <tiny-button-group :data="categories" v-model="categories_value"></tiny-button-group>
    </div>
    <div class="groupbtn">
        <div class="groupbtntitle">
            排序类型
        </div>
        <tiny-button-group class="group_btn" :data="sorting" v-model="sorting_value"></tiny-button-group>
    </div>
    <div class="groupbtn">
        <div class="groupbtntitle">
            正序/倒序
        </div>
        <tiny-button-group class="group_btn" :data="order" v-model="order_value"></tiny-button-group>
    </div>
    <div class="groupbtn">
        <div class="groupbtntitle">
            图片比例
        </div>
        <tiny-button-group class="group_btn" :data="ratio" v-model="ratio_value"></tiny-button-group>
    </div>
    <div class="groupbtn">
        <div class="groupbtntitle">
            图片分辨率
        </div>
        <tiny-button-group class="group_btn" :data="resolutions" v-model="resolutions_value"></tiny-button-group>
    </div>
    <div class="setting1">
        <tiny-button class="settingbtn" type="primary" @click="setting">确定</tiny-button>
    </div>
        
</div>

<right-menu></right-menu>
</template>

<style scoped>

.new-loading .tiny-loading__spinner .tiny-loading__text {
  color: white;
}

.setting1{
    display: flex;
    justify-content: center;
    width: 100vw;
    box-sizing: border-box;
    padding: 20px;
}
.settingbtn{
    display: flex;
    width: 100px;
    justify-content: center;
    border: none;
}
.groupbtn{
    display: flex;
    width: 100vw;
    box-sizing: border-box;
    padding: 10px;
}

.groupbtntitle{
    width: 100px;
    height: 28px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 12px;
    background: white;
    margin:0 30px;
    
}
.setting{
    position: absolute;
    z-index: 200;
    top: 0px;
    left: 0px;
    width: 100vw;
    height: 100vh;
    background: rgba(230,230,230,0.5);
    border-radius: 15px;
    display: flex;
    justify-content: start;
    align-content: center;
    flex-wrap: wrap;
    box-sizing: border-box;
    padding: 50px;
}

.btn{
    width: 100px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 5px;
    font-size: 13px;
    cursor: pointer;
    box-sizing: border-box;
    padding: 5px 10px;
    border: none;
}

.btn:hover{
    box-shadow: 0px 2px 5px 2px rgba(23,23,23,1);
}

.btnbar{
    position: absolute;
    z-index: 200;
    bottom: 0;
    width: 100vw;
    height: 60px;
    /* background-color: white; */
    display: flex;
    align-items: center;
    justify-content: center;
}

.wallpaper{
    position: relative;
    width: 100vw;
    height: 100vh;
    padding: 0;
    margin: 0;
    overflow:hidden;
    background-color: black;
    border-radius: 15px;
}
.img{
    object-fit:cover;
    width: 100vw;
    height: 100vh;
}

.pre{
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
    z-index: 99;
    height: 100vh;
    top:0;
    left: 0;
    width: 40px;
    background-color: white;
    opacity: 0.2;
    /* border-radius: 15px; */
    border-top-left-radius: 15px;
    border-bottom-left-radius: 15px;
}
.pre:hover{
    opacity: 0.5;
}

.next {
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
    z-index: 99;
    height: 100vh;
    top: 0;
    right: 0;
    background-color: white;
    opacity: 0.2;
    /* border-radius: 15px; */
    border-top-right-radius: 15px;
    border-bottom-right-radius: 15px;
}

.next:hover{
    opacity: 0.5;
}

.pimg{
    width: 40px;
    height: auto;
}
</style>