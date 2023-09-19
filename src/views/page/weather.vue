<script lang="ts" setup>
import { onMounted,ref,reactive} from 'vue';
import { getpoi,getWeather } from '../../api';
import 'qweather-icons/font/qweather-icons.css'
import { location,nowWeather } from '../../types'
import { appWindow } from '@tauri-apps/api/window'

const weather = reactive({
    name:'',
    id:'',
    now:{} as nowWeather,
    color:'255,255,255',
    opacity:70
});

onMounted(()=>{
    let storageWeather = getStorageWeather();
    weather.id = storageWeather.id
    weather.name = storageWeather.name;
    weather.color = storageWeather.color
    weather.opacity = storageWeather.opacity
    
    // 初始化获取天气信息
    getWeather(weather.id).then((res:any)=>{
        console.log(res);
        if(res.data.code == '200'){
            console.log(res.data);
            weather.now = res.data.now;
        }
    })

    // 定时获取天气信息 1h
    setInterval(function(){
        console.log(weather)
        if(weather.name){
            getWeather(weather.id).then((res:any)=>{
                console.log(res);
                if(res.data.code == '200'){
                    console.log(res.data);
                    weather.now = res.data.now;
                }
            })
        }else{
        }
    },30*60*1000);
})

// input城市
const city = ref<string>('');
// 输入检索控制
const citySetTimeout=ref<any>();
// 城市列表选择
const citys = reactive<location[]>([]);
// 搜索框显示控制
const show = ref<boolean>(false);
// input 函数
const inputChange = function(){
    clearTimeout(citySetTimeout.value);
    citySetTimeout.value = setTimeout(function(){
        if(!city.value){
            citys.splice(0,citys.length);
            return
        }
        getpoi(city.value).then((res:any)=>{
            if(res.data.code == '200'){
                citys.splice(0,citys.length);
                citys.push(...res.data.location as []);
            }else{
                citys.splice(0,citys.length);
            }
            console.log(citys)
        });
    },300)
}

// 选择后获取城市天气
const selectCity = function(item:any){
    weather.name = item.name;
    weather.id = item.id;
    citys.splice(0,citys.length);
    getWeather(item.id).then((res:any)=>{
        if(res.data.code == '200'){
            weather.now = res.data.now;
            saveWeather()
        }
        show.value = !show.value;
    })
}

//#region storage
const saveWeather = function(){
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(weather));
}

const getStorageWeather = function(){
    let str:string|null = localStorage.getItem(appWindow.label+'-data');
    console.log(str)
    if(str){
        return JSON.parse(str);
    }else{
        localStorage.setItem(appWindow.label+'-data',JSON.stringify(weather));
        return weather;
    }
}

//#endregion

// 右键菜单
import rightMenu from '../../components/rightMenu.vue';
const rightClick = function(){
    show.value = false
    transparentbool.value = false
    console.log(123);
}

// 监听窗口移动和禁止选择
import { windowPosition,forbidSelect } from './../../common/index'
windowPosition();
forbidSelect();
appWindow.setResizable(false);


// 颜色选择
const colorChange = function(){
    document.getElementById('colorBoard')?.click()
}

// 获取颜色
const colorInput = function(e:any){
    const {red,green,blue } = hexToRgba(e.srcElement.value,1);
    weather.color = red+','+green+','+blue
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(weather))
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
    weather.opacity = e.srcElement.value
    localStorage.setItem(appWindow.label+'-data',JSON.stringify(weather))
}
</script>

<template>
    <right-menu @right-click="rightClick" :border-radius="'25px'">
        <img class="rightmenu-img" :src="'icon/poi.png'" @click="show = !show">
        <img class="rightmenu-img" :src="'icon/transparent.png'" @click="transparentbool = !transparentbool">
        <img class="rightmenu-img" :src="'icon/changecolor.png'" @click="colorChange" >
    </right-menu>
    <!-- 城市选择 -->
    <div class="weather-select" v-show="show">
        <div class='citysearch'>
            <tiny-input v-model="city" @input="inputChange" placeholder="搜索城市"></tiny-input>
        </div>
        <div style="overflow: scroll;width: 180px;height:calc(200px - 70px);display: flex;justify-content: center;flex-wrap: wrap;align-content: start;">
            <div v-for="item in citys" class="citylist" @click="selectCity(item)">
                {{ item.country +' '+ item.adm1+' '+ item.adm2 +' ' + item.name }}
            </div>
        </div>
    </div> 
    <!-- 天气展示 -->
    <div class="weather" :style="{background:'rgba('+weather.color+','+weather.opacity/100+')'}">
        <div class="weather-city" >
                {{ weather.name?weather.name:'城市' }}
        </div>
        <div class="weather-icon-div">
            <div class="weather-temp">
                {{ weather.now.text?weather.now.text:'未知' }}  {{ weather.now.temp?weather.now.temp+'°':'0°' }}
            </div>
            <div style="display: flex;justify-content: center;width: 100px;">
                <i :class="['qi-'+(weather.now.icon?weather.now.icon:100),'weather-icon']"></i>
            </div>

            <div class="weather-wind">
                <div style="width: 100px;display: flex;justify-content: center;letter-spacing:0px ;">
                    {{ weather.now.windDir?weather.now.windDir:'未知' }}
                </div>
                <div style="width: 100px;display: flex;justify-content: center;letter-spacing:0px ;">
                    {{ weather.now.windScale?weather.now.windScale+"级":'0级' }}
                </div>
            </div>
            <div class="weather-water">
                <div style="width: 100px;display: flex;justify-content: center;letter-spacing:3px">
                    {{ '降水量' }}
                </div>
                <div style="width: 100px;display: flex;justify-content: center;letter-spacing:0px ;">
                    {{  weather.now.precip?weather.now.precip+'mm':'0mm' }}
                </div>
                
            </div>
        </div>
    </div>

    <!-- 背景色选择 -->
    <input type="color" v-show="true" id="colorBoard" class="color-board" @input="colorInput">
    <!-- 透明度 -->
    <input type="range" v-show="transparentbool" @input="rangeChange" id="transparent" class="transparent" step="1" max="100" min="0">
</template>

<style>
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

.weather{
    display: flex;
    flex-direction: row;
    justify-content: center;
    flex-wrap: wrap;
    /* background: rgba(217,217,217,0.1); */
    width: 200px;
    height: 200px;
    border-radius: 25px;
}
.weather-icon-div{
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    align-items: center;
    height: 150px;
}
.weather-city{
    display: flex;
    width: 200px;
    font-size: 20px;
    align-items: center;
    justify-content: center;
    height: 50px;
}
.weather-icon{
    font-size: 40px;

}
.weather-temp{
    display: flex;
    justify-content: center;
    font-size: 20px;
    width: 100px;
}
.weather-wind{
    width: 100px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
}
.weather-water{
    width: 100px;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
}

.weather-select{
    position: absolute;
    z-index: 99;
    display: flex;
    flex-direction: row;
    justify-content: center;
    flex-wrap: wrap; 
    width: 200px;
    height: 200px;
}
.citysearch{
    margin-top: 15px;
    width: 180px;
}
.citylist{
    display: flex;
    width: 180px;
    font-size: 10px;
    padding-left: 10px;
    padding-right: 10px;
    padding-top: 3px;
    padding-bottom: 3px;
    box-sizing: border-box;
    text-align: left;
    border-bottom: 1px solid gray;
    border-right: 1px solid gray;
    border-left:1px solid gray;
    cursor:pointer;
    background: rgba(217,217,217,0.8);
}
::-webkit-scrollbar {
    display: none;
}

.rightmenu{
    width: 100vw;
    height: 100vh;
    background: rgba(217,217,217,0.5);
    position: absolute;
    top: 0px;
    border-radius: 25px;
    padding: 10px;
    box-sizing: border-box;
    text-align: center;
    z-index: 10;
    display: flex;
    justify-content: center;
    align-items: center ;
    flex-wrap: wrap;
}

.rightmenu-img{
    width: 25x;
    height: 25px;
    display: flex;
}
</style>