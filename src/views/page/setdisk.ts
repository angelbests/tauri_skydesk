import { listen } from '@tauri-apps/api/event';
import { disk, memory, newchart } from './sysinfo.vue';

// 磁盘
export const setdisk = async function () {
let mycharts = [];
await listen("disk", (event: any) => {
disk.splice(0, disk.length);
for (let i = 0; i < event.payload.length; i++) {
let diskobject = JSON.parse(event.payload[i]) as string;
diskobject.available_space = (diskobject.available_space / 1024 / 1024 / 1024).toFixed(0);
diskobject.total_space = (diskobject.total_space / 1024 / 1024 / 1024).toFixed(0);
disk.push(diskobject);
}

if (mycharts.length == 0) {
for (let i = 0; i < disk.length; i++) {
let option = {
grid: {
left: '0%',
right: '0%',
bottom: 0,
top: 0
},
xAxis: {
type: "value",
min: 0,
max: memory.total_memory,
show: true
},
yAxis: {
data: ['Memory'],
show: true,
},
series: [
{
type: "bar",
data: [memory.used_memory],
}
]
};
let mychart = newchart('disk' + (i + 1), option, 500, 60);
mycharts.push(mychart);
}
}

console.log(disk);


});
};
