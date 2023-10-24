<script lang="ts" setup>
import { onMounted, reactive, ref} from 'vue';
import { listen } from '@tauri-apps/api/event'
import rightMenu from '../../components/rightMenu.vue';
import * as echarts from 'echarts';

const version:any = ref("");
const memory = reactive({
    total_memory:"",
    used_memory:""
})

const swap = reactive({
    total_swap:"",
    used_swap:""
})

const disk:any[] = reactive([]);

const cpu:any[] = reactive([]);

onMounted(async ()=>{
    // 系统版本
    await listen('version',(event)=>{
        version.value = JSON.parse(event.payload as string).version
    })

    setmemory();
    setswap();
    setdisk();
    setcpu();
});

//cpu
const setcpu = async function () {
    let mycharts:any[] = [];
    await listen('cpu',(event:any)=>{
        for(let i=0;i<event.payload.length;i++){
            let one = JSON.parse(event.payload[i])
            if(cpu[i] == undefined){
                cpu.push(one);
                cpu[i]["data"] = [0,0,0,0,0,0,0,0,0,0];
            }
            cpu[i].data.splice(0,1);
            cpu[i].data.push(one.usage)
        }
        // alert(cpu.length)
        if(mycharts.length == 0){
            for(let i=0;i<cpu.length;i++){
                let option = {
                    xAxis: {
                        type: 'category',
                        boundaryGap: false,
                        data: [1,2,3,4,5,6,7,8,9,10],
                        show:false,
                    },
                    yAxis: {
                        boundaryGap: [0, '10%'],
                        type: 'value', 
                        show:true,
                        interval:10,
                        min:0,
                        max:100
                    },
                    series: [
                        {
                            name:'cpu',
                            data: cpu[i].data,
                            type: 'line',
                            // smooth: true,
                            symbol: 'none',
                            areaStyle: {
                            },
                        }
                    ]
                }
                let mychart = newchart('cpu'+i,option,160,160) 
                mycharts.push(mychart);
            }
        }

        for(let i=0;i<mycharts.length;i++){
            mycharts[i].setOption({
                xAxis: {
                    type: 'category',
                    boundaryGap: false,
                    data: [1,2,3,4,5,6,7,8,9,10],
                    show:false,
                },
                yAxis: {
                    boundaryGap: [0, '10%'],
                    type: 'value', 
                    show:false,
                    interval:10,
                    min:0,
                    max:100
                },
                series: [
                    {
                        name:'cpu',
                        data: cpu[i].data,
                        type: 'line',
                        // smooth: true,
                        symbol: 'none',
                        areaStyle: {
                        },
                    }
                ]
            })
        }

    })
    console.log(cpu)
}

// 磁盘
const setdisk = async function () {
    let mycharts = [];
    await listen("disk",(event:any)=>{
        disk.splice(0,disk.length)
        for(let i=0;i<event.payload.length;i++){
            let diskobject = JSON.parse(event.payload[i]) as string;
            diskobject.available_space = (diskobject.available_space/1024/1024/1024).toFixed(0);
            diskobject.total_space = (diskobject.total_space/1024/1024/1024).toFixed(0);
            disk.push(diskobject);
        }

        if (mycharts.length==0){
            for(let i=0;i<disk.length;i++){
                let option = {
                    grid:{
                        left:'0%',
                        right:'0%',
                        bottom:0,
                        top:0
                    },
                    xAxis:{
                            type:"value",
                            min:0,
                            max:disk[i].total_space,
                            show:false
                    },
                    yAxis:{
                        data: ['disk'],
                        show:false,
                    },
                    series:[ 
                        {
                            type:"bar",
                            name:"total_swap",
                            data:[disk[i].total_space-disk[i].total_space],
                            showBackground: true,
                            label:{
                                show:true,
                                // formatter:'{@total_swap} GB',
                                formatter:(res:any)=>{
                                    if(((disk[i].total_space-disk[i].available_space)/disk[i].total_space)<0.3){
                                        return `${disk[i].kind.split(":")[0]}盘 ${res.value} GB`
                                    }
                                    return res.value+' GB';
                                },
                                offset:[170,0]
                            },
                            barGap:'-100%',
                            silent:true
                        },
                        {
                            type:"bar",
                            name:"used_swap",
                            data:[disk[i].available_space],
                            showBackground: true,
                            label:{
                                show:true,
                                // formatter:'虚拟内存 使用 {@used_swap} GB'
                                formatter:(res:any)=>{
                                    if(((disk[i].total_space-disk[i].available_space)/disk[i].total_space)>=0.3){
                                        return `${disk[i].kind.split(":")[0]}盘 使用 ${res.value} GB`
                                    }
                                    return res.value+' GB';
                                }
                            },
                        },
                        
                    ]
                };
                let mychart=newchart('disk'+i,option,500,40);
                mycharts.push(mychart);
            }
        }

        for(let i=0;i<mycharts.length;i++){
            let option = {
                grid:{
                    left:'0%',
                    right:'0%',
                    bottom:0,
                    top:0
                },
                xAxis:{
                        type:"value",
                        min:0,
                        max:disk[i].total_space,
                        show:false
                },
                yAxis:{
                    data: ['disk'],
                    show:false,
                },
                series:[ 
                    {
                        type:"bar",
                        name:"total_swap",
                        data:[disk[i].total_space],
                        showBackground: true,
                        barGap:'-100%',
                        silent:true
                    },
                    {
                        type:"bar",
                        name:"used_swap",
                        data:[disk[i].total_space - disk[i].available_space],
                        showBackground: true,
                    },
                    
                ]
            };
            mycharts[i].setOption(option);
        }


    })
    console.log(disk)
}

const setmemory =async function(){
    let option = {
        grid:{
            left:'0%',
            right:'0%',
            bottom:0,
            top:0
        },
        xAxis:{
                type:"value",
                min:0,
                max:memory.total_memory,
                show:false
        },
        yAxis:{
            data: ['Memory'],
            show:false,
        },
        series:[ 
            {
                type:"bar",
                name:"total_memory",
                data:[memory.total_memory],
                showBackground: true,
                label:{
                    show:true,
                    // formatter:'{@total_memory} GB',
                    formatter:(res:any)=>{
                        if((memory.used_memory/memory.total_memory)<0.3){
                            return `物理内存 ${res.value} GB`
                        }
                        return res.value+' GB';
                    },
                    offset:[170,0]
                },
                barGap:'-100%',
                silent:true
            },
            {
                type:"bar",
                name:"used_memory",
                data:[memory.used_memory],
                showBackground: true,
                label:{
                    show:true,
                    // formatter:'物理内存 使用 {@used_memory} GB'
                    formatter:(res:any)=>{
                        if((memory.used_memory/memory.total_memory)>=0.3){
                            return `物理内存 使用 ${res.value} GB`
                        }
                        return res.value+' GB';
                    }
                },
            },
        ]
    }
    let mychart = newchart('memory',option,500,40);

        // 系统内存
    await listen("memory",(event)=>{
        memory.total_memory = (JSON.parse(event.payload as string).totalmemory/1024/1024/1024).toFixed(1);
        memory.used_memory = (JSON.parse(event.payload as string).usedmemory/1024/1024/1024).toFixed(1);
        let option = {
            grid:{
                left:'0%',
                right:'0%',
                bottom:0,
                top:0
            },
            xAxis:{
                    type:"value",
                    min:0,
                    max:memory.total_memory,
                    show:false
            },
            yAxis:{
                data: ['Memory'],
                show:false,
            },
            series:[ 
                {
                    type:"bar",
                    name:"total_memory",
                    data:[memory.total_memory],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'{@total_memory} GB',
                        formatter:(res:any)=>{
                            if((memory.used_memory/memory.total_memory)<0.3){
                                return `物理内存 ${res.value} GB`
                            }
                            return res.value+' GB';
                        },
                        offset:[170,0]
                    },
                    barGap:'-100%',
                    silent:true
                },
                {
                    type:"bar",
                    name:"used_memory",
                    data:[memory.used_memory],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'物理内存 使用 {@used_memory} GB'
                        formatter:(res:any)=>{
                            if((memory.used_memory/memory.total_memory)>=0.3){
                                return `物理内存 使用 ${res.value} GB`
                            }
                            return res.value+' GB';
                        }
                    },
                },
                
            ]
        }
        mychart.setOption(option)
    })

    
}

const setswap =async function(){
    let option = {
        grid:{
            left:'0%',
            right:'0%',
            bottom:0,
            top:0
        },
        xAxis:{
                type:"value",
                min:0,
                max:swap.used_swap,
                show:false
        },
        yAxis:{
            data: ['swap'],
            show:false,
        },
        series:[ 
                {
                    type:"bar",
                    name:"total_swap",
                    data:[swap.total_swap],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'{@total_swap} GB',
                        formatter:(res:any)=>{
                            if((swap.used_swap/swap.total_swap)<0.3){
                                return `虚拟内存 ${res.value} GB`
                            }
                            return res.value+' GB';
                        },
                        offset:[170,0]
                    },
                    barGap:'-100%',
                    silent:true
                },
                {
                    type:"bar",
                    name:"used_swap",
                    data:[swap.used_swap],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'虚拟内存 使用 {@used_swap} GB'
                        formatter:(res:any)=>{
                            if((swap.used_swap/swap.total_swap)>0.3){
                                return `虚拟内存 使用 ${res.value} GB`
                            }
                            return res.value+' GB';
                        }
                    },
                },
                
            ]
    }
    let mychart = newchart('swap',option,500,40);

    await listen("swap",(event)=>{
        swap.total_swap = (JSON.parse(event.payload as string).totalswap/1024/1024/1024).toFixed(1);
        swap.used_swap = (JSON.parse(event.payload as string).usedswap/1024/1024/1024).toFixed(1);
        let option = {
            grid:{
                left:'0%',
                right:'0%',
                bottom:0,
                top:0
            },
            xAxis:{
                    type:"value",
                    min:0,
                    max:swap.total_swap,
                    show:false
            },
            yAxis:{
                data: ['swap'],
                show:false,
            },
            series:[ 
                {
                    type:"bar",
                    name:"total_swap",
                    data:[swap.total_swap],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'{@total_swap} GB',
                        formatter:(res:any)=>{
                            if((swap.used_swap/swap.total_swap)<0.3){
                                return `虚拟内存 ${res.value} GB`
                            }
                            return res.value+' GB';
                        },
                        offset:[170,0]
                    },
                    barGap:'-100%',
                    silent:true
                },
                {
                    type:"bar",
                    name:"used_swap",
                    data:[swap.used_swap],
                    showBackground: true,
                    label:{
                        show:true,
                        // formatter:'虚拟内存 使用 {@used_swap} GB'
                        formatter:(res:any)=>{
                            if((swap.used_swap/swap.total_swap)>0.3){
                                return `虚拟内存 使用 ${res.value} GB`
                            }
                            return res.value+' GB';
                        }
                    },
                },
                
            ]
        }
        mychart.setOption(option)
    })

    
}

const newchart = function(id:string,option:any,width:any,height:any){
    let myChart = echarts.init(document.getElementById(id),null,{
        width:width,
        height:height 
    });
    myChart.setOption(option);
    return myChart;
}

const getdate = function(){
    let date =new Date();
    let year = date.getFullYear();
    let month = date.getMonth()+1;
    let day = date.getDate();
    let hour = date.getHours()
    let min = date.getMinutes();
    let sec = date.getSeconds();
    return year + '-' + month + '-' + day + ' ' + hour + ':' + min + ':' + sec
}

</script>

<template>
<right-menu></right-menu>
<div class="bg">
    <div class="version" id="version">
        {{ version }}
    </div>
    <div class="memory" id="memory">
    </div>
    <div class="swap" id="swap">
    </div>
    <div v-for="(item,index) in disk" :id="'disk'+index" class="disk">
    </div>
    <div class="cpus">
        <div v-for="(item,index) in cpu" :id="'cpu'+index">

        </div>
    </div>
</div>
</template>

<style scoped>
.bg{
    width: 100vw;
    height: 100vh;
}

.version{
    width: 100vw;
    height: 30px;
    display: flex;
    align-items: center;
    background-color: white;
    justify-content: center;
}
.memory{
    width: 100vw; 
    height: auto;
    display: flex;
    align-items: center;
    background-color: white;
    justify-content: center;
}
.swap{
    width: 100vw; 
    height: auto;
    display: flex;
    align-items: center;
    background-color: white;
    justify-content: center;
}
.disk{
    width: 100vw; 
    height: auto;
    display: flex;
    align-items: center;
    background-color: white;
    justify-content: center;
}
.cpus{
    width: 100vw;
    height: auto;
    display: flex;
    flex-wrap: wrap;
}
</style>