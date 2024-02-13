<script lang="ts" setup>
import {  onMounted, reactive, ref } from 'vue';
const col = ref(4);
const waterfall:{
    top:number
    left:number
    row:number
    col:number
}[] = reactive([])
onMounted(()=>{
// 通过父元素拿到子元素的渲染数量;
// 

    let div = document.getElementById("waterfall-div")


})
</script>

<template>
    <div id="waterfall" class="waterfall">
        <div id="waterfall-div" class="waterfall-div">
            <div class="waterfall-item" v-for="item in 20">
                {{ item }}
            </div>
        </div>
    </div>
</template>

<style scoped>
.waterfall{
    width: 100%;
    height: 100%;
    overflow: hidden;
    overflow-y: scroll;
    position: relative;
    background-color: white;
}
.waterfall-div{
    width: 100%;
    position: relative;

}
.waterfall-item{
    background-color: gray;
    width: 200px;
    height: 200px;
}
</style>