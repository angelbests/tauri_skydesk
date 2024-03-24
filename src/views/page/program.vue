<script lang="ts" setup>
import { onMounted,reactive,ref} from 'vue';
import { getLnk,uuid } from "./../../common/index"
import { writeBinaryFile,BaseDirectory,copyFile,removeFile,FileEntry,exists  } from "@tauri-apps/api/fs"
import { appDataDir,basename,dataDir} from '@tauri-apps/api/path'
import { invoke,convertFileSrc } from '@tauri-apps/api/tauri'
import rightMenu from './../../components/rightMenu.vue'
import { forbidSelect } from "./../../common/index"
import { Loading } from '@opentiny/vue';
forbidSelect();
const load:any = ref(0);
onMounted(async()=>{
    load.value = Loading.service({
        text:'扫描程序文件中...',
        background: 'rgba(0, 0, 0, 0.3)',
        size:'large',
        target:document.getElementById('lnks-container')
    })
    let data1 = await getLnk();
    let data2 = await getLnk(await dataDir()+"Microsoft\\Windows\\Start Menu\\Programs");
    data.push(...data1,...data2);
    for(let i = 0;i<data.length;i++){
        if(data[i].path.indexOf("Windows PowerShell")>=0){
            continue;
        }
        let exesrcstr:string = await invoke("getlnk2",{path:data[i].path});
        let exesrc:string = JSON.parse(exesrcstr).local_base_path;
        if(exesrc!="false"&& (exesrc.slice(-3).toLowerCase() == 'exe') ){
            if(isHaveChina(exesrc)||isHaveChina(JSON.parse(exesrcstr).ico_location)){
                exesrc = JSON.parse(exesrcstr).ico_location;
            }
            if(exesrc.indexOf('MY_COMPUTER\\')>=0){
                exesrc = exesrc.substring(12,200);
            }
            let filename =await basename(data[i].path);
            let iconsrc:string =await addsrc(exesrc) as string;
            if(iconsrc){
                icons.push({
                    name:filename.split('.')[0],
                    ico:iconsrc == './note/exe.png'?'./note/exe.png':convertFileSrc(iconsrc),
                    path:exesrc
                }); 
            }

        }
    }
    load.value.close();
})
const icons:{
    name:string,
    ico:string,
    path:string,
}[] = reactive([]);
const data:FileEntry[] =reactive([])
const addsrc =async function(path:string){
    // 获取程序exe的图标
    let arr:any = "";
    if(isHaveChina(path)){
        let filename = uuid()+'.exe'
        await copyFile(path,filename,{ dir:BaseDirectory.AppData })

        let filesrc =  await appDataDir()+filename;
        if(await exists(filesrc)){
            arr = await invoke("geticon",{path:filesrc});
            await removeFile(filesrc);
        }else{
            return '';
        }
    }else{
        if(await exists(path)){
            arr = await invoke("geticon",{path:path});
        }else{
            return '';
        }
    }  

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
                let name = uuid() + '.png';
                await writeBinaryFile(name, new Uint8Array(ico[j].data), { dir: BaseDirectory.AppData });
                return await appDataDir() + name;
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
                if(width>=maxsize){
                    if(iconame!=""){
                        let src = await appDataDir() + iconame;
                        await removeFile(src);
                    }
                    maxsize = width;
                    iconame = name; 
                }
            }
        }
        if(iconame == "" ){
            return "./note/exe.png";
        }else{
            return await appDataDir() + iconame;
        }
        
    }else{ 
        for(let i=0;i<group_icon.length;i++){
            let num = group_icon[i].data.slice(4,5)[0];
            for(let z=0;z<num;z++){
                if (ico[z].type == 'png'){
                    let name = uuid() + '.png';
                    await writeBinaryFile(name, new Uint8Array(ico[z].data), { dir: BaseDirectory.AppData });
                    return await appDataDir() + name;
                }
                let name = uuid() + '.ico';
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
                if (width>=maxsize){
                    if(iconame!=""){
                        let src = await appDataDir() + iconame;
                        await removeFile(src);
                    }
                    maxsize = width;
                    iconame = name;
                } 
            } 
        } 
        if(iconame == "" ){
            return "./note/exe.png";
        }else{
            return await appDataDir() + iconame;
        }
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

const dragstart = function(event:DragEvent,name:string,ico:string,path:string){
    event.dataTransfer?.setData('lnk',JSON.stringify({
        icon:ico,
        src:path,
        title:name,
        type:'exe'
    }));
}

const dragover = function(event:DragEvent){
    event.preventDefault();
}

import { appWindow } from "@tauri-apps/api/window"
const close = function(){
    appWindow.close();
}

</script>

<template>
<div class="title-bar" data-tauri-drag-region style="flex-direction: row-reverse;">
    <div style="width: 30px;height: 30px;display: flex;align-items: center;justify-content: center;" @click="close">
        <img :src="'bar/close.png'" style="width: 20px;height: 20px;">
    </div>
</div>
<div class="lnks-container" id="lnks-container">
    <div class="lnk-scroll">
        <div v-for="item in icons" draggable="true" @dragover="dragover($event)" @dragstart="dragstart($event,item.name,item.ico,item.path)" class="lnk">
            <div class="lnk-ico">
                <img draggable="false" :src="item.ico" class="lnk-ico-img">
            </div>
            <div class="lnk-title">
                {{ item.name }}
            </div>
        </div>
    </div>
</div>
<right-menu :border-radius="'0'" :zindex="9999"></right-menu>
</template>

<style>
html,body{
    background-color: white;
    padding: 0 !important;
    margin: 0 !important;
    border: 0 !important;
    overflow: hidden;
}
.title-bar{
    width: 400px;
    height: 30px;
    background-color: rgba(240,240,220,1);
    display: flex;
    position: fixed;
    z-index: 80;
}
.lnks-container {
    width: 100vw;
    height: calc(100vh - 30px);
    padding-top: 30px;
    overflow-y: scroll;
    overflow-x: hidden;
}
.lnk-scroll {
    width: 100vw;
    height: auto;
    display: flex;
    flex-wrap: wrap;
    flex-direction: row;
    align-items: flex-start;
    align-content: start;
}
.lnk {
    width: 80px;
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
}
.lnk:hover {
    background-color: rgba(240,240,250,0.5);
    border-radius: 5px;
}
.lnk-ico{
    width: 80px;
    height: 40px;
    display: flex;
    justify-content: center;
    align-items: center;
}
.lnk-ico-img{
    width: 30px;
    height: 30px;
    display: flex;
    justify-content: center;
    align-items: center;
}
.lnk-title {
    font-size: 10px;
    width: 70px;
    height: 20px;
    display: flex;
    justify-content: center;
    text-overflow: clip;
    white-space: nowrap;
    overflow: hidden;
    padding: 3px 3px;
    box-sizing: border-box;
}
::-webkit-scrollbar{
    display: none;
}

</style>