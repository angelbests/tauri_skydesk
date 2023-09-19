import { windowType } from '../types'
import { WebviewWindow,appWindow } from "@tauri-apps/api/window"
import { exit } from '@tauri-apps/api/process';
import { Command } from '@tauri-apps/api/shell';
import { onMounted } from 'vue'

export const uuid = function():string {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function (c) {
        let r = Math.random() * 16 | 0,
            v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

// 创建窗口
export const createWindow = function(name:string,windowConfig:windowType):string{
    let windowName = name +'-'+ uuid()
    new WebviewWindow(windowName,windowConfig);
    saveWindow(windowName);
    // 保存窗口设置信息
    localStorage.setItem(windowName,JSON.stringify(windowConfig));
    return windowName;
}

// 保存窗口的label
export const saveWindow = function(windowName:string):boolean{
    let windowsStr:string|null =  localStorage.getItem('windows');
    if(windowsStr){
        let windows = JSON.parse(windowsStr);
        windows.push(windowName);
        localStorage.setItem("windows",JSON.stringify(windows))
    }else{
        localStorage.setItem("windows",'["'+windowName+'"]');
    }
    return true;
}

// 监听并记录窗口坐标
export function windowPosition(){
    onMounted(()=>{
        appWindow.listen('tauri://move',async (event:any)=>{
            console.log(event);
            let windowStr:string|null = localStorage.getItem(appWindow.label);
            if(windowStr){
                let window = JSON.parse(windowStr);
                const factor = await appWindow.scaleFactor();
                // 解决多屏切换异常问题
                // if(event.payload.x/factor  == -25600 || event.payload.y/factor == -25600){
                //     return;
                // };
                window.x = event.payload.x/factor;
                window.y = event.payload.y/factor;
                localStorage.setItem(appWindow.label,JSON.stringify(window));
            }
        })
    })
}


// 禁止文本选择
export function forbidSelect(dom:any = document){
    onMounted(()=>{
        dom.addEventListener("selectstart", function (e:any) {
            e.preventDefault();
        });
    })
}

//初始化窗口
export function initWindow(){
    let windowsStr = localStorage.getItem('windows')
    if(windowsStr){
        let windows = JSON.parse(windowsStr);
        windows.forEach( function(element:string) {
            // 获取窗口信息
            let strwin = localStorage.getItem(element);
            if(strwin){
                let win = JSON.parse(strwin);
                // win.skipTaskbar = false;
                new WebviewWindow(element,win)
            }
        });
    }
}

// 检查进程列表是否存在相同应用在运行中，如果有则退出程序
// 配置 shell
// "shell": {   
//     "all": true,  
//     "scope": [
//       {
//         "name": "tasklist",
//         "cmd": "tasklist"
//       }
//     ] 
//   },
export const checkRun =  async function (){
    // 新建命令对象
    let command = new Command('tasklist')
    // 命令执行逻辑
    let num = 0;
    command.stdout.on('data', (data:any)=>{
        let n =  data.indexOf("skydesk");
        if(n>=0){
            num = num + 1; 
        }
        if(num>=2){
            exit(1);
        }
    });
    // 执行命令
    const child = await command.spawn();
    console.log(child);

}

export const gettime = function(){
    let date = new Date();
    let y  =date.getFullYear();
    let m = date.getMonth()+1;
    let d = date.getDate(); 
    let h = date.getHours();
    let min = date.getMinutes();
    let s = date.getSeconds();
    return y+'-'+m+'-'+d+' '+h+':'+min+':'+s;
}

export const hideRightMenu = function(){
    document.addEventListener(
        "contextmenu",
        function(e){
            e.preventDefault()
        }
    )
}
