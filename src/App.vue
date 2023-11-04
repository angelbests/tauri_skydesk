<script setup lang="ts">
import { onMounted } from 'vue';
import { createDir, BaseDirectory } from '@tauri-apps/api/fs';
import { checkRun } from './common/index';

// import { Command } from '@tauri-apps/api/shell';
// import { resourceDir } from '@tauri-apps/api/path';
import { enable } from 'tauri-plugin-autostart-api'
onMounted(async ()=>{
    checkRun() // 检查是否已存在进程 防止多起
    await createDir('skydesk', { dir: BaseDirectory.AppData, recursive: true }); // 创建数据文件夹 Roming
    await enable();     // 自启
    // disable(); //取消自启 引入包disable,isEnabled

    // 原本使用cmd 设置开机自启  但找到插件就改为插件自启
    // let resourceDirPath = await resourceDir();
    // resourceDirPath = resourceDirPath.substring(4,resourceDirPath.length);
    // let command = new Command('reg',["add","HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run","/v","skydesk","/t","REG_SZ","/d",resourceDirPath+'skydesk.exe']);
    // await command.spawn();  
    // localStorage.clear()
})

</script>

<template>
    <router-view></router-view>
</template>
