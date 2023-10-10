<script lang="ts" setup>
import { onMounted,ref,reactive } from 'vue';
import { LogicalSize, appWindow,WebviewWindow,getAll } from '@tauri-apps/api/window';
import { copyFile,BaseDirectory,writeBinaryFile,removeFile} from '@tauri-apps/api/fs'
import { basename,appDataDir } from '@tauri-apps/api/path';
import { invoke,convertFileSrc } from '@tauri-apps/api/tauri';
import { open as dialogOpen } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/shell'
import { createWindow,uuid,initWindow } from './../common/index'
import { lnk,editorType,windowType } from '../types';
import { gettime } from './../common/index'
import {confirm } from '@tauri-apps/api/dialog';
import { relaunch } from '@tauri-apps/api/process';

//#region 初始化
appWindow.setResizable(false);
onMounted(async ()=>{
    await invoke('systeminfo');
    await invoke('netspeed');
    await invoke('wheelclick');
    document.addEventListener("contextmenu",function(ev:MouseEvent){
        ev.preventDefault();
    })
 
    document.addEventListener("selectstart",function(ev:Event){
        ev.preventDefault();
    })

    // 初始化editor数据
    let editorDataStr = localStorage.getItem('editorData');
    if(editorDataStr){
        let data = JSON.parse(editorDataStr);
        editorData.push(...data);
    }

    // localStorage.setItem('roulette',JSON.stringify(roulettelnks))
    let roulettelnksstr = localStorage.getItem('roulette');
    roulettelnks.splice(0,roulettelnks.length)
    if(roulettelnksstr){
        roulettelnks.push(...JSON.parse(roulettelnksstr))
    }else{
        const newlnks:lnk[] = [
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
            }
        ]
        roulettelnks.push(...newlnks);
    }

    let collapsestr = localStorage.getItem('collapseActive');
    if(collapsestr){
        collapseArr.value = JSON.parse(collapsestr)
    }

    let lnksstr = localStorage.getItem('lnks');
    if(lnksstr){
        collapselnks.splice(0,collapselnks.length)
        collapselnks.push(...JSON.parse(lnksstr))
    }

    initWindow();

    // 监听缓存变动
    window.addEventListener('storage',function(e:StorageEvent){
        console.log(e)
        if(e.key&&e.key?.indexOf('editor')==0){
            editorData.filter((item:{
                window:windowType,
                editor:editorType,
                label:string
            },index:number)=>{
                // editor缓存数据变化记录
                if(item.label+'-data' == e.key){
                    if(e.newValue){
                        let data:editorType = JSON.parse(e.newValue);
                        editorData[index].editor.value = data.value;
                        editorData[index].editor.color = data.color;
                        editorData[index].editor.opacity = data.opacity;
                    }
                }else{
                    // 窗口缓存数据变化记录
                    if(e.newValue){
                        if(localStorage.getItem(item.label as string)){
                            editorData[index].window = JSON.parse(localStorage.getItem(item.label as string) as string)
                        }
                    }
                }
                localStorage.setItem('editorData',JSON.stringify(editorData))
            })
        }
    })
}) 
//#endregion

//#region right bar
const minimize = function(){
    appWindow.minimize();
}

const toggle = ref<boolean>(false);
const maximize = function(){
    toggle.value = !toggle.value
    appWindow.toggleMaximize()
}

const close = function(){
    appWindow.hide()
}

//#endregion
  
//#region apps切换
const appsbool = ref<boolean>(true);
// circlebool
const circlebool = ref<boolean>(true); 
     
const circlefn = function(){             
    if(!circlebool.value){ 
        appWindow.setSize(new LogicalSize(400,600));
        appWindow.setAlwaysOnTop(false)
        circlebool.value = !circlebool.value
    }else{    
        circlebool.value = !circlebool.value  
        appWindow.setSize(new LogicalSize(50,30)); 
        appWindow.setAlwaysOnTop(true)
    }                           
}                                                    
//#endregion

//#region editor的内容
const editorData = reactive<{
    window:windowType,
    editor:editorType,
    label:string
}[]>([])

// 打开新note窗口
const addNote = function(){
    let winconfig = {  
        url:"/#/sub/editor",      
        fileDropEnabled:true,      
        alwaysOnTop:false,
        decorations:false, 
        transparent:true,
        height:400,     
        width:400,   
        skipTaskbar:true               
    }
   let label = createWindow("editor",winconfig); 
    editorData.push({
        window:winconfig,
        editor:{
            color:'255,255,255',
            opacity:100,
            value:'',
            date:gettime(),
        },
        label:label
    })
    localStorage.setItem('editorData',JSON.stringify(editorData))
}

// 在关闭单独窗口后再次打开 或 聚焦
const openeditor =async function(editor:{
    editor:editorType,
    window:windowType,
    label:string
}){
    let windows = getAll();
    let arr = windows.filter((item)=>{
        if(item.label == editor.label){
            return true
        }
        return false
    })
    if(arr.length == 0){
        editorData.filter((item,index)=>{
            if(item.label == editor.label){
                let label = createWindow('editor',editor.window as windowType);
                editorData.splice(index,1,{
                    editor:{
                        color:editor.editor.color,
                        value:editor.editor.value,
                        opacity:editor.editor.opacity,
                        date:editor.editor.date,
                    },
                    label:label,
                    window:editor.window
                })
                localStorage.setItem(label+'-data',JSON.stringify({
                    color:editor.editor.color,
                    value:editor.editor.value,
                    opacity:editor.editor.opacity,
                    date:editor.editor.date,
                }))
            }
        })
        localStorage.setItem('editorData',JSON.stringify(editorData))
    }else{
       await arr[0].setFocus()
    }
}

// 关闭窗口清理缓存和数据
const closeeditor = function(editor:{
    editor:editorType,
    window:windowType,
    label:string
}){
    let windows = getAll()
    console.log(windows)
    windows.forEach((window:any)=>{
        if(window.label == editor.label){
            window.close()
            localStorage.removeItem(window.label)
            localStorage.removeItem(window.label+'-data')

            let windowsStr = localStorage.getItem('windows')
            if(windowsStr){
                let windows = JSON.parse(windowsStr);
                windows.splice(windows.indexOf(appWindow.label),1)
                localStorage.setItem('windows',JSON.stringify(windows))
            }
        }

    })
    editorData.filter((item,index)=>{
        if(item.label == editor.label){
            editorData.splice(index,1)
        }
    })
    localStorage.setItem('editorData',JSON.stringify(editorData))
}

//#endregion

//#region exec程序执行 图标添加
const show = ref(false);
const openExe =async function(item:lnk){
    if(item.type=="exe"){
        await open(item.src)
    }else if(item.type=="tauricommand"){
        await invoke(item.src)
    }
} 

interface collapselnksType {
    title:string,
    name:string,
    lnks:lnk[]
}

const collapselnks = reactive<collapselnksType[]>(
    [
        {
            title:'程序',
            name:'1',
            lnks:[
                {
                    icon:'note/notepad.png',
                    src:'C:/Windows/notepad.exe',
                    title:'记事本',
                    type:'exe'
                },
                {
                    icon:'note/screen.png',
                    src:'screen',
                    title:'截屏',
                    type:"tauricommand"
                },
                {
                    icon:'note/resource-manager.png',
                    src:'C:/Windows/explorer.exe',
                    title:'资源管理器',
                    type:'exe'
                },
                {
                    icon:'note/regedit.png',
                    src:'C:/Windows/regedit.exe',
                    title:'注册表',
                    type:'exe'
                },
                {
                    icon:'note/calc.png',
                    src:'C:/Windows/system32/calc.exe',
                    title:'计算器',
                    type:'exe'
                },
                {
                    icon:'note/cmd.png',
                    src:'C:/Windows/system32/cmd.exe',
                    title:'CMD', 
                    type:'exe'
                },
                {
                    icon:'note/computer-configuration.png',
                    src:'C:/Windows/system32/compmgmt.msc',
                    title:'计算机管理',
                    type:'exe'
                },
                {
                    icon:'note/control-panel.png',
                    src:'C:/Windows/system32/control.exe',
                    title:'控制面板',
                    type:'exe'
                },
                {
                    icon:'note/disk.png',
                    src:'C:/Windows/system32/diskmgmt.msc',
                    title:'磁盘管理',
                    type:'exe'
                },
                {
                    icon:'note/Policy-Group.png',
                    src:'C:/Windows/system32/gpedit.msc',
                    title:'本地策略组',
                    type:'exe'
                },
                {
                    icon:'note/Computer-information.png',
                    src:'C:/Windows/system32/msinfo32.exe',
                    title:'系统信息',
                    type:'exe'
                },
                {
                    icon:'note/remote-desktop.png',
                    src:'C:/Windows/system32/mstsc.exe',
                    title:'远程桌面',
                    type:'exe'
                },
                {
                    icon:'note/keyboard.png',
                    src:'C:/Windows/system32/osk.exe',
                    title:'屏幕键盘',
                    type:'exe'
                },
                {
                    icon:'note/Volume.png',
                    src:'C:/Windows/system32/SndVol.exe',
                    title:'音量合成器',
                    type:'exe'
                },
                {
                    icon:'note/task-manager.png',
                    src:'C:/Windows/system32/Taskmgr.exe',
                    title:'任务管理器',
                    type:'exe'
                },
                {
                    icon:'note/firewall.png',
                    src:'C:/Windows/system32/WF.msc',
                    title:'防火墙',
                    type:'exe'
                }
            ]
        }
    ]
        );

const lnkcollapse = ref('程序');
const addshow = function(collapse:string){
    lnkcollapse.value = collapse;
    show.value = true
}

// 选择添加执行文件路径
const lnksrc = ref('');
const addsrc =async function(){
    console.log(1)
    const selected:any = await dialogOpen({
        multiple:false,
        filters:[{
            name:'程序路径',
            extensions:['exe']
        }] 
    });
    lnksrc.value = selected;
    let exename = await basename(selected); 
    lnktitle.value  = exename.split('.')[0];
    // 获取程序exe的图标
    let arr:any = "";
    if(isHaveChina(selected)){
        let filesrc =  await appDataDir()+'a.exe';
        await copyFile(selected,'a.exe',{ dir:BaseDirectory.AppData })
        arr = await invoke("geticon",{path:filesrc});
        await removeFile(filesrc);
    }else{
        arr = await invoke("geticon",{path:selected});
    }
    console.log(arr);   

    let group_icon = arr[0];
    let ico = arr[1];
    for(let i=0;i<group_icon.length;i++){
        group_icon[i] = JSON.parse(group_icon[i]);
        group_icon[i].data= JSON.parse(group_icon[i].data);
    }

    for(let j=0;j<ico.length;j++){
        ico[j] = JSON.parse(ico[j]);
        ico[j].data = JSON.parse(ico[j].data);
    }

    // 0, 0,  保留的字节
    // 1, 0,  资源类型
    // 1, 0,  图象个数
    //64, 图标宽度
    //64, 图标宽度
    //0,  图标宽度
    //0,  未用 
    //0, 0, 0, 0,  保留的
    //40, 66, 0, 0, 图象数据块的长度
    // 22, 0, 0, 0, 图象数据块相对于文件头部的偏移量
    // 0, 0, 1, 0, 1, 0, 48, 48, 0, 0, 0, 0, 0, 0, 168, 37, 0, 0, 22, 0, 0, 0,
    
    let maxsize = 0;
    let iconame = "";
    if(group_icon.length == 0){
        for(let j=0;j<ico.length;j++){
            if(ico[j].type == 'png'){
                let name = uuid() + '.ico';
                await writeBinaryFile(name, new Uint8Array(ico[j].data), { dir: BaseDirectory.AppData });
                maxsize = 1000;
                iconame = name
            }else{
                // 40, 0, 0, 0,   BMP 信息头结构长度
                // 32, 0, 0, 0,   图像宽度
                // 64, 0, 0, 0,   图像高度（XOR图高度＋AND图高度）
                // 1, 0,          位面板数
                // 32, 0,         每象素所占位数
                // 0, 0, 0, 0,    象素数据的压缩类型　
                // 0, 16, 0, 0,   图象数据的长度
                // 35, 46, 0, 0,35, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,  未用

                let width = ico[j].data.slice(4,5)[0];
                let height = ico[j].data.slice(4,5)[0];
                let datalength = ico[j].data.slice(20,24);
                let header = [0, 0, 1, 0, 1, 0,width,height, 0, 0, 0, 0, 0, 0, ...datalength, 22, 0, 0, 0,];
                let name = uuid() + '.ico';
                await writeBinaryFile(name, new Uint8Array(header.concat(ico[j].data)), { dir: BaseDirectory.AppData });
                console.log(width);
                if(width>maxsize){
                    maxsize = width;
                    iconame = name;
                }
            }
        }
        lnkicon.value = await appDataDir() + iconame;
    }else{ 
        for(let i=0;i<group_icon.length;i++){
            let num = group_icon[i].data.slice(4,5)[0];
            for(let z=0;z<num;z++){
                let name = uuid() + '.ico';
                if (ico[z].type == 'png'){
                    await writeBinaryFile(name, new Uint8Array(ico[z].data), { dir: BaseDirectory.AppData });
                    // await writeBinaryFile(ico[z].name+'.ico', new Uint8Array(ico[z].data), { dir: BaseDirectory.AppData });
                    maxsize = 1000;
                    iconame = name;
                    continue;
                }
                let icodata = group_icon[i].data.slice(6+14*z,6+14*(z+1));
                let header = [0, 0, 1, 0, 1, 0];
                let width = icodata[0];
                header.push(width);
                let height = icodata[1];
                header.push(height);
                // let color = icodata[6];
                header.push(...[0,0,0,0,0,0]);
                let datalength = icodata.slice(8,12);
                header.push(...datalength);
                let offest = [22,0,0,0];
                header.push(...offest);
                await writeBinaryFile(name, new Uint8Array(header.concat(ico[z].data)), { dir: BaseDirectory.AppData });
                // await writeBinaryFile(ico[z].name+'.ico', new Uint8Array(header.concat(ico[z].data)), { dir: BaseDirectory.AppData });

                if (width>maxsize){
                    maxsize = width;
                    iconame = name;
                }
            }
        }
        console.log(iconame)
        lnkicon.value = await appDataDir() + iconame;
    }
}

function isHaveChina(str:string) {
	if (escape(str).indexOf("%u") < 0) {
        // 不含中文
		return false
	} else {
		// 包含中文;
        return true;
	}
}
 
// 选择添加图标
const lnkicon = ref('');
const addicon =async function(){
    console.log(1)
    let selected:any = await dialogOpen({
        filters:[{
            name:'图标路径', 
            extensions:['png','svg','jpg','jpeg','ico']
        }]
    });
    let fileName =await basename(selected);
    await copyFile(selected,fileName ,{ dir:BaseDirectory.AppData })
    lnkicon.value = await appDataDir()+fileName
    // lnkicon.value = convertFileSrc(await appDataDir()+fileName) 
}

const lnktitle = ref('');
const addlnk = function(){
    let collapseindex = 1; 
    collapselnks.filter((item,index)=>{
        if(item.title == lnkcollapse.value){
           collapseindex = index 
           return true
        }
    })

    if(lnkicon.value&&lnksrc.value||lnktitle.value.length>1){
        collapselnks[collapseindex].lnks.push({
            title:lnktitle.value,
            src:lnksrc.value, 
            icon:convertFileSrc(lnkicon.value),
            type:"exe"
        })
    }
    lnkcollapse.value = ''
    lnkicon.value = ''
    lnksrc.value = ''
    lnktitle.value = ''
    show.value=false
    localStorage.setItem('lnks',JSON.stringify(collapselnks))
}

// 新增合集
const addcollapseshow = ref(false);
const collapsename = ref('');
const addcollapse = function(){
    if(collapsename.value=='') return
    let name = collapselnks[collapselnks.length-1].name
    name = (parseInt(name) + 1).toString()
    collapselnks.push({
        title:collapsename.value,
        name:name,
        lnks:[]
    })
    collapsename.value = '';
    addcollapseshow.value = false
    localStorage.setItem('lnks',JSON.stringify(collapselnks))
}

// 删除合集
const deletecollapse =async function(index:number){
    let data = await confirm('确定删除合集？', {
        title:'删除合集提示',
        okLabel:'删除',
        cancelLabel:'取消'
    });
    if(data){
        collapselnks.splice(index,1);
        localStorage.setItem('lnks',JSON.stringify(collapselnks))
    }
}

const collapseArr = ref([]);
const collapsechange = function(arr:[]){
    collapseArr.value = arr;
    localStorage.setItem('collapseActive',JSON.stringify(collapseArr.value));
}

//#endregion

//#region 拖拽
const dragstart = function(event:DragEvent,item:lnk,index:number,collapse:string){
    event.dataTransfer?.setData('index',index.toString());
    event.dataTransfer?.setData('lnk',JSON.stringify(item));
    event.dataTransfer?.setData('collapse',collapse);
}

const dragover = function(event:DragEvent){
    event.preventDefault();
}

// 同合集拖拽互换位置
const drop = function(event:DragEvent,item:lnk,index:number,collapse:string){
    console.log(item);
    event.preventDefault();
    let dragcollapse:any = event.dataTransfer?.getData('collapse');

    if(dragcollapse == collapse){
        let str:any = event.dataTransfer?.getData('index');
        if(!str) return;
        let dragIndex = parseInt(str);
        if(dragIndex !== index){
            let collapseindex = 0;
            collapselnks.filter((item,index)=>{
                if(item.title == collapse){
                    collapseindex = index;
                    return true;
                }
            })

            let dragData = collapselnks[collapseindex].lnks[dragIndex];
            collapselnks[collapseindex].lnks[dragIndex] = collapselnks[collapseindex].lnks[index];
            collapselnks[collapseindex].lnks[index] = dragData;
            localStorage.setItem('lnks',JSON.stringify(collapselnks))
        }
    }
} 

const dragaddlnk = function(event:DragEvent,collapse:string){
    event.preventDefault();
    let dragcollapse:any = event.dataTransfer?.getData('collapse');
    if(dragcollapse != collapse){
        let lnkstr = event.dataTransfer?.getData('lnk');
        if(!lnkstr) return
        let draglnk:lnk = JSON.parse(lnkstr);
        collapselnks.filter((item,index)=>{
            if(item.title == collapse){
                let arr = collapselnks[index].lnks.filter((item)=>{
                    return item.src == draglnk.src
                })
                if(arr.length == 0){
                    collapselnks[index].lnks.push(draglnk)
                }
            }
        })
        localStorage.setItem('lnks',JSON.stringify(collapselnks))
    }
}

const dropDelete = function(event:DragEvent,collapse:string){
    let str:any = event.dataTransfer?.getData('index')
    let dragcollapse:any = event.dataTransfer?.getData('collapse');
    let collapseindex = 0;
    collapselnks.filter((item,index)=>{
        if(item.title == dragcollapse){
            collapseindex = index;
            return true;
        }
    })
    if(str&&dragcollapse == collapse){
        let index:number = parseInt(str);
        collapselnks[collapseindex].lnks.splice(index,1);
        localStorage.setItem('lnks',JSON.stringify(collapselnks))
    }
}

//#endregion

//#region 创建文件夹
const show2 = ref(false)
const row = ref(2);
const col = ref(2);
const addFloder =function(){
    createWindow('folder',{
        width:row.value*70+10,
        height:col.value*70+10,
        transparent:true,
        url:'/#/sub/lnk/'+row.value+'/'+col.value,
        decorations:false,
        alwaysOnTop:false,
        fileDropEnabled:false,
        skipTaskbar:true
    })
    show2.value = !show2.value
    row.value = 2
    col.value = 2
}
//#endregion

//#region  weather

const openweather = function(){
    let winconfig:windowType = {
        title:"weather",
        width:200,
        height:200,
        decorations:false,
        transparent:true,
        url:'/#/sub/weather',
        skipTaskbar:true,
        alwaysOnTop:false
    }
    createWindow('weather',winconfig)
}
//#endregion

//#region search

const opensearch = function(){
    let winconfig:windowType = {
        title:"search",
        width:200,
        height:200,
        decorations:false,
        transparent:true,
        url:'/#/sub/search',
        skipTaskbar:true,
        alwaysOnTop:false
    }
    createWindow('search',winconfig)
}
 
//#endregion

//#region roulette  轮盘代码区
const roulettelnks = reactive<lnk[]>([]);

const droproulette = function(event:DragEvent){
    event.preventDefault();
    let str:any = event.dataTransfer?.getData('lnk');
    if(roulettelnks.length<4){
        console.log(roulettelnks.length)
        let lnk = JSON.parse(str)
        let arr = roulettelnks.filter(item=>{
            return item.src == lnk.src
        })
        if(arr.length==0){
            roulettelnks.push(lnk)
            localStorage.setItem('roulette',JSON.stringify(roulettelnks))
        }
    }
}

 const droprouletteitem = function(event:DragEvent,index:number){
    event.preventDefault();
    let str:any = event.dataTransfer?.getData('lnk');
    if(str){
        let lnk = JSON.parse(str);
        let arr = roulettelnks.filter(item=>{
            return item.src == lnk.src
        })
        if(arr.length == 0){
            roulettelnks[index].icon = lnk.icon;
            roulettelnks[index].src = lnk.src;
            roulettelnks[index].title = lnk.title;
            roulettelnks[index].type = lnk.type;
            localStorage.setItem('roulette',JSON.stringify(roulettelnks))
        }
    }
 }

//#endregion

//#region netspeed
 const opennetspeed = function(){
    createWindow('netspeed',{
        title:"netspeed",
        width:90,
        height:50,
        decorations:false,
        transparent:true,
        url:'/#/sub/netspeed',
        skipTaskbar:true,
        alwaysOnTop:false
    })
 }
//#endregion

//#region  程序信息
const info =async function(){
    let data = await confirm('作者：skymiao \n邮箱：angelbests1314@163.com', {
        title:'skydesk',
        okLabel:'重置',
        cancelLabel:'确定'
    });
    if(data){
        localStorage.clear();
        await relaunch();
    }
}
//#endregion

//#region  wallpaper
const openwallpaper = function(){
    new WebviewWindow('wallpaper',{
        title:"wallpaper",
        width:500,
        height:500,
        decorations:false,
        transparent:true,
        url:'/#/sub/wallpaper',
        skipTaskbar:true,
        alwaysOnTop:false
    })
}
//#endregion


//#region  折叠面板



//#endregion
</script> 

<template>
<div class="index" :style="{width:circlebool?'100vw':'50px',height:circlebool?'100vh':'30px'}">
    <div v-if="!circlebool" class="button" @dblclick="circlefn" data-tauri-drag-region>
        <img :src="'bar/dog.png'" class="bar-img" alt="">
    </div>
    <div v-if="circlebool" class="bar" >
        <div class="bar-left" data-tauri-drag-region>
            <div class="button"  @dblclick="circlefn" >
                <img :src="'bar/dog.png'" class="bar-img" alt="">
            </div>
            <div class="button" @click="appsbool=!appsbool" >
                <img :src="'bar/apps.png'" class="bar-img" alt="">
            </div>
            <!-- <div class="button" @click="info">
                <img :src="'icon/setting2.png'" class="bar-img" >
            </div>  -->
            <div class="button" @click="info">
                <img :src="'icon/info2.png'" class="bar-img" >
            </div> 
        </div>  
 
        <!-- 通用bar按钮 -->
        <div class="bar-right">
            <div v-if="false" class="button" @click="minimize">
                <img :src="'bar/minimize.png'" class="bar-img" alt="">
            </div>
            <div v-if="false" class="button" @click="maximize">
                <img :src="toggle?'bar/maximize.png':'bar/restore.png'" class="bar-img" alt="">
            </div>
            <div class="button" @click="close">
                <img :src="'bar/close.png'" class="bar-img" alt="">
            </div>
        </div>
    </div>
    <div v-if="circlebool" class="container">
        <div v-for="(item,index) in editorData"  :style="{background:'rgba('+item.editor.color+','+item.editor.opacity/100+')',order:1000-index}" class="card" >
            <div :style="{height: '20px',width: '100%',display: 'flex',alignItems: 'center'}">
                <div style="width: calc(100% - 20px);">
                    {{ item.editor.date }}
                </div>
                <img :src="'icon/close.png'" style="width: 20px;height: 20px;" @click="closeeditor(item)" />
            </div>
            <div v-html="item.editor.value" :style="{height:'calc( 100% - 20px )',overflow:'scroll'}" @click="openeditor(item)"></div>
        </div>
       <div class="card-add" @click="addNote" :style="{order:-1}">
            <img draggable="false" :src="'note/add.png'" style="pointer-events: none;"  alt="">
        </div>
    </div>

    <!-- app -->
    <div v-if="circlebool" class="app" :style="{top:appsbool?'calc(0px - (100vh - 30px))':'30px'}">
        <div class="apps">
            <tiny-collapse @change="collapsechange" v-model="collapseArr" style="width: 100vw;">
                <tiny-collapse-item v-for="(collapse,collapseindex) in collapselnks" :title="collapse.title" :name="collapse.name" >
                    <template  #title-right>
                        <tiny-tag v-if="collapseindex == 0" @click.stop="addcollapseshow = true">新增合集</tiny-tag>
                        <tiny-tag v-if="collapseindex != 0" @click.stop="deletecollapse(collapseindex)">删除合集</tiny-tag>
                    </template>
                    <div draggable="true" @dragover="dragover($event)"  class="apps" @drop.stop="dragaddlnk($event,collapse.title)">
                        <div draggable="true" @dragstart="dragstart($event,item,index,collapse.title)" @dragover="dragover($event)" 
                        @drop="drop($event,item,index,collapse.title)" v-for="(item,index) in collapse.lnks" class="menu-item" @click="openExe(item)">
                            <img draggable="false" class="menu-item-img" :src="item.icon">
                            <div class="menu-item-title">
                                {{ item.title }}
                            </div>
                        </div>
                        <div @click="addshow(collapse.title)" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/add.png'">
                            <div class="menu-item-title">
                                {{ '新增' }}
                            </div>
                        </div>
                        <div @dragover="dragover($event)" @drop="dropDelete($event,collapse.title)" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/garbage.png'">
                            <div class="menu-item-title">
                                {{ '删除' }}
                            </div>
                        </div>
                    </div>
                </tiny-collapse-item>
                <tiny-collapse-item title="桌面组件" name="99">
                    <div class="apps">
                        <div @click="show2 = true" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/folder.png'">
                            <div class="menu-item-title">
                                {{ '快捷合集' }}
                            </div>
                        </div>
                        <div @click="openweather" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/weather.png'">
                            <div class="menu-item-title">
                                {{ '天气' }}
                            </div>
                        </div>
                        <div @click="opensearch" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/search.png'">
                            <div class="menu-item-title">
                                {{ '搜索栏' }}
                            </div>
                        </div>
                        <div @click="opennetspeed" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/netspeed.png'">
                            <div class="menu-item-title">
                                {{ '网速' }}
                            </div>
                        </div>
                        <div @click="openwallpaper" class="menu-item">
                            <img draggable="false" class="menu-item-img" :src="'/note/wallpaper.png'">
                            <div class="menu-item-title">
                                {{ '壁纸' }}
                            </div>
                        </div>
                    </div>

                </tiny-collapse-item>  
            </tiny-collapse>
        </div>
        <div class="roulette" draggable="true" @dragover="dragover($event)" @drop="droproulette($event)" >
            <div class="menu-item" v-for="(item,index) in roulettelnks" @drop="droprouletteitem($event,index)">
                <img draggable="false" class="menu-item-img" :src="item.icon">
                <div class="menu-item-title">
                    {{ item.title }}
                </div>
            </div>
        </div>
    </div>
    
</div>

<!-- 添加lnk -->
<tiny-dialog-box v-model:visible="show" :show-close="false" title="添加快捷信息" :center="true" :append-to-body="true" fullscreen>
    <tiny-input class="input" readonly clearable v-model="lnkcollapse" @click="addsrc" :disabled="true"></tiny-input>
    <tiny-input class="input" readonly clearable placeholder="选择程序路径" v-model="lnksrc" @click="addsrc"></tiny-input>
    <tiny-input class="input" readonly clearable placeholder="选择程序图标" v-model="lnkicon" @click="addicon"></tiny-input>
    <tiny-input class="input" placeholder="程序名称" v-model="lnktitle"></tiny-input>
    <template #footer>
        <tiny-button type="primary" @click="addlnk"> 确定 </tiny-button>
        <tiny-button  @click="show = false" > 取消 </tiny-button>
    </template>
</tiny-dialog-box>

<!-- 添加新folder -->
<tiny-dialog-box v-model:visible="show2" :show-close="false" title="合集大小" :center="true" width="60%" top="20%" :append-to-body="true" fullscreen>
        <tiny-numeric style="width: 80%;margin:0 10%;" size="mini" width="20px" v-model="col" min="1" max="20" mouseWheel unit="行"></tiny-numeric>
        <tiny-numeric style="width: 80%;margin:0 10%;" size="mini" width="20px" v-model="row" min="1" max="20" mouseWheel unit="列"></tiny-numeric>
    <template #footer>
        <tiny-button type="primary" @click="addFloder"> 确定 </tiny-button>
        <tiny-button  @click="show2 = false" > 取消 </tiny-button>
    </template>
</tiny-dialog-box>

<!-- 添加新collapse -->
<tiny-dialog-box v-model:visible="addcollapseshow" :show-close="false" title="新建合集" :center="true" width="60%" top="20%" :append-to-body="true" fullscreen>
    <tiny-input class="input" clearable placeholder="输入合集名称" v-model="collapsename"></tiny-input>
    <template #footer>
        <tiny-button type="primary" @click="addcollapse"> 确定 </tiny-button>
        <tiny-button  @click="addcollapseshow = false" > 取消 </tiny-button>
    </template>
</tiny-dialog-box>

</template>

<style>

.tiny-collapse-item__content{
    padding: 0 !important;
    border: none;
}

.input{
    margin-bottom: 20px;
}

body{
    overflow: hidden;
}
.index{
    display: flex;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    position: relative;
    transition: all 0.2s linear;
}

/* apps */

.app{
    position: absolute;
    z-index: 90;
    width: 100vw;
    height: calc( 100% - 30px );
    background: white;
    transition: top 0.2s linear;
    display: flex;
    flex-wrap: wrap;
}

.apps{
    position: relative;
    width: 100vw;
    height: calc( 100% - 90px );
    background: white;
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-start;
    align-content: flex-start;
    overflow-x:hidden ;
    overflow-y: scroll;
} 

.roulette{
    position: relative;
    width: 100vw;
    height: 90px;
    background: rgba(123,123,123, 0.2);
    display: flex;
    align-items: center;
    justify-content: center;
}

.menu-item {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    width: 80px;
    margin-top: 10px;
    border-radius: 5px;
    padding: 5px 5px;
    box-sizing: border-box;
}

.menu-item:hover{
    background: floralwhite;
}

.menu-item-img{
    display: flex;
    width: 35px;
    height: 35px;
    pointer-events: none;
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

/* note */
.container{
    top: 30px;
    position:absolute;
    z-index: 80;
    width: 100vw;
    height:calc(100vh - 30px);
    background-color: wheat;
    overflow-y: scroll;
    overflow-x: hidden;
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-content: start;
}

.card{
    width: calc(100vw - 20px);
    height: 200px;
    /* background-color: blueviolet; */
    margin-top: 10px;
    border-radius: 5px;
    box-sizing: border-box;
    padding: 10px;
    overflow: hidden;
}

.card:first-child{
    margin-bottom: 10px;
}

.card-add{
    display: flex;
    width: calc(100vw - 20px);
    height: 60px;
    justify-content: center;
    /* border: 1px solid black; */
    border-radius: 5px;
    margin-top: 10px;
    box-sizing: border-box;
    padding: 5px;
    background-color: white;
}

/* .card-add:hover{
    background: gray;
} */

::-webkit-scrollbar{
    display: none;
}

/* bar */

.bar{
    position: absolute;
    z-index: 100;
    width: 100vw;
    height: 30px;
    background-color: floralwhite;
    display: flex;
}

/* left-bar */
.bar-left{
    width: calc( 100vw - 120px );
    height: 30px;
    display:flex;
}
 

/* right-bar */
.bar-right{
    width: 120px;
    display: flex;
    justify-content: end;
}

.button{
    display: flex;
    height: 30px;
    width: 40px;
    align-items: center;
    justify-content: center;
    background-color: floralwhite;
    border-radius: 5px;
}

.button:hover{
    background-color: white;
}

.bar-img{
    pointer-events: none;
    width: 15px;
    height: 15px;
}

</style>