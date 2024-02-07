<script lang="ts" setup>
import { LogicalSize, appWindow } from "@tauri-apps/api/window";
import rightMenu from "../../components/rightMenu.vue";
import { onMounted, reactive, ref, watch } from "vue";
import { wallpaperType } from "./../../types";
import { hideRightMenu, forbidSelect } from "./../../common/index";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { fetch, ResponseType } from "@tauri-apps/api/http";
import {
  writeBinaryFile,
  BaseDirectory,
  createDir,
  exists,
} from "@tauri-apps/api/fs";
import { pictureDir, appDataDir, basename } from "@tauri-apps/api/path";
hideRightMenu();
forbidSelect();
appWindow.setSize(new LogicalSize(1000, 600));
appWindow.setResizable(false);
const wallpapers = reactive<wallpaperType[]>([]);
const wallpaper = reactive({
  downloadsrc: "",
  httpsrc: "./wallpaper/a.jpg",
});

onMounted(async () => {
  // 创建用户图片文件夹
  await createDir("skydesk", { dir: BaseDirectory.Picture, recursive: true });
  await createDir("wallpaper", { dir: BaseDirectory.AppData, recursive: true });
});

const getwallpaper = async function (page: number) {
  let href = "https://wallhaven.cc/api/v1/search?purity=100";
  // categories  sorting  order  resolutions  ratios
  if (categories_value.value != "") {
    href = href + "&categories=" + categories_value.value;
  }
  if (sorting_value.value != "") {
    href = href + "&sorting=" + sorting_value.value;
  }
  if (order_value.value != "") {
    href = href + "&order=" + order_value.value;
  }
  if (ratio_value.value != "") {
    href = href + "&ratios=" + ratio_value.value;
  }
  if (resolutions_value.value != "") {
    href = href + "&resolutions=" + resolutions_value.value;
  }
  href = href + "&page=" + page;
  const response: any = await fetch(href, {
    method: "GET",
    timeout: 10000,
  });
  if (response.data.data == undefined) {
    appWindow.close();
  } else {
    wallpapers.push(...response.data.data);
  }
  console.log(wallpapers);
};

const savewallpaper = async function (path: string) {
  let filename = await basename(path);
  wallpaper.downloadsrc = (await appDataDir()) + "wallpaper\\" + filename;
  let bool = await exists(wallpaper.downloadsrc);
  if (!bool) {
    let response: any = await fetch(path, {
      method: "GET",
      timeout: 1000,
      headers: {
        ContentType: "application/octet-stream",
      },
      responseType: ResponseType.Binary,
    });
    await writeBinaryFile(
      "skydesk\\" + filename,
      new Uint8Array(response.data),
      { dir: BaseDirectory.Picture }
    );
  }
  wallpaper.httpsrc = convertFileSrc(wallpaper.downloadsrc);
  return true;
};

//#region 设置壁纸
const setwallpaper = async function (item: wallpaperType) {
  let filename = await basename(item.path as string);
  let bool = await exists("skydesk\\" + filename, {
    dir: BaseDirectory.Picture,
  });
  if (!bool) {
    await savewallpaper(item.path as string);
  }
  let picturesrc = (await pictureDir()) + "skydesk\\" + filename;
  await invoke("setwallpaper", { src: picturesrc });
};
//#endregion

const downloadimg = async function () {
  await setting();
  for (let i = 0; i < wallpapers.length; i++) {
    console.log("start");
    let filename = await basename(wallpapers[i].path as string);
    let bool = await exists("skydesk\\" + filename, {
      dir: BaseDirectory.Picture,
    });
    if (!bool) {
      await savewallpaper(wallpapers[i].path as string);
    }
    console.log("end");
  }
  console.log(1);
};

//#region 设置
const page = ref(1);
const categories_value = ref("");
const sorting_value = ref("");
const order_value = ref("");
const ratio_value = ref("");
const resolutions_value = ref("");
const setting_show = ref(false);
const setting = async function () {
  wallpapers.length = 0;
  setting_show.value = false;
  for (let i = 1; i <= page.value; i++) {
    await getwallpaper(i);
  }
};

// 大类
const categories = reactive([
  { text: "动漫", value: "010" },
  { text: "常规", value: "100" },
  { text: "人物", value: "001" },
  { text: "空", value: "" },
]);

// 排序
const sorting = reactive([
  { text: "相关性", value: "relevance" },
  { text: "随机", value: "random" },
  { text: "浏览量", value: "views" },
  { text: "下载量", value: "favorites" },
  { text: "流行", value: "toplist" },
  { text: "日期", value: "date added" },
  { text: "热度", value: "hot" },
  { text: "空", value: "" },
]);

// 正序 倒序
const order = reactive([
  { text: "正序", value: "asc" },
  { text: "倒序", value: "desc" },
  { text: "空", value: "" },
]);

// 分辨率
const ratio = reactive([
  {
    text: "5:4",
    value: "5:4",
    resolutions: [
      { text: "1280x1024", value: "1280x1024" },
      { text: "1600x1280", value: "1600x1280" },
      { text: "1920x1536", value: "1920x1536" },
      { text: "2560x2048", value: "2560x2048" },
      { text: "3840x3072", value: "3840x3072" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "4:3",
    value: "4:3",
    resolutions: [
      { text: "1280x960", value: "1280x960" },
      { text: "1600x1200", value: "1600x1200" },
      { text: "1920x1440", value: "1920x1440" },
      { text: "2560x1920", value: "2560x1920" },
      { text: "3840x2880", value: "3840x2880" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "16:10",
    value: "16:10",
    resolutions: [
      { text: "1280x800", value: "1280x800" },
      { text: "1600x1000", value: "1600x1000" },
      { text: "1920x1200", value: "1920x1200" },
      { text: "2560x1600", value: "2560x1600" },
      { text: "3840x2400", value: "3840x2400" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "16:9",
    value: "16:9",
    resolutions: [
      { text: "1280x720", value: "1280x720" },
      { text: "1600x900", value: "1600x900" },
      { text: "1920x1080", value: "1920x1080" },
      { text: "2560x1440", value: "2560x1440" },
      { text: "3840x2160", value: "3840x2160" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "Ultrawide",
    value: "Ultrawide",
    resolutions: [
      { text: "2560x1080", value: "2560x1080" },
      { text: "3840x1440", value: "3840x1440" },
      { text: "3840x1600", value: "3840x1600" },
      { text: "空", value: "" },
    ],
  },
  {
    text: "空",
    value: "",
    resolutions: [{ text: "空", value: "" }],
  },
]);

const resolutions = reactive<
  {
    text: string;
    value: string;
  }[]
>([{ text: "空", value: "" }]);

watch(ratio_value, (newValue: string) => {
  ratio.filter((item, index, arr) => {
    if (item.value == newValue) {
      resolutions.length = 0;
      resolutions.push(...arr[index].resolutions);
      resolutions_value.value = resolutions[0].value;
    }
  });
});
//#endregion

const showsetting = function () {
  setting_show.value = true;
  // 模拟右击事件
  let body = document.getElementsByTagName("body")[0];
  var evObj = document.createEvent("MouseEvents");
  evObj.initMouseEvent(
    "contextmenu",
    true,
    true,
    window,
    1,
    12,
    345,
    7,
    220,
    false,
    false,
    false,
    false,
    0,
    null
  );
  body.dispatchEvent(evObj);
};
import { IconDownload, IconCustom } from "@opentiny/vue-icon";
</script>

<template>
  <div class="wallpapers-div">
    <div class="wallpapers">
      <div
        :style="{
          width: '230px',
          height: (230 / Number(item.ratio)).toFixed(0) + 'px',
          position: 'relative',
          display: 'inline',
        }"
        v-for="(item, index) in wallpapers"
      >
        <img
          :id="'img-' + index"
          :src="item.thumbs?.original"
          :style="{
            width: '230px',
            height: (230 / Number(item.ratio)).toFixed(0) + 'px',
          }"
        />
        <div class="imgbtn"
        :style="{
            width: '230px',
            height: (230 / Number(item.ratio)).toFixed(0) + 'px',
            top:(20-230 / Number(item.ratio)).toFixed(0) + 'px'
        }">
          <tiny-button
            :icon="IconDownload()"
            @click="savewallpaper(item.path as string)"
            type="warning"
            circle
          ></tiny-button>
          <tiny-button
            :icon="IconCustom()"
            @click="setwallpaper(item)"
            type="warning"
            circle
          ></tiny-button>
        </div>
      </div>
    </div>
  </div>
  <div class="setting" v-show="setting_show">
    <div class="groupbtn">
      <div class="groupbtntitle">图片分类</div>
      <tiny-button-group
        :data="categories"
        v-model="categories_value"
      ></tiny-button-group>
    </div>
    <div class="groupbtn">
      <div class="groupbtntitle">排序类型</div>
      <tiny-button-group
        class="group_btn"
        :data="sorting"
        v-model="sorting_value"
      ></tiny-button-group>
    </div>
    <div class="groupbtn">
      <div class="groupbtntitle">正序/倒序</div>
      <tiny-button-group
        class="group_btn"
        :data="order"
        v-model="order_value"
      ></tiny-button-group>
    </div>
    <div class="groupbtn">
      <div class="groupbtntitle">图片比例</div>
      <tiny-button-group
        class="group_btn"
        :data="ratio"
        v-model="ratio_value"
      ></tiny-button-group>
    </div>
    <div class="groupbtn">
      <div class="groupbtntitle">图片分辨率</div>
      <tiny-button-group
        class="group_btn"
        :data="resolutions"
        v-model="resolutions_value"
      ></tiny-button-group>
    </div>
    <div class="groupbtn">
      <div class="groupbtntitle">查询页数</div>
      <tiny-numeric
        :min="1"
        :max="40"
        v-model="page"
        placeholder="请输入页数"
      ></tiny-numeric>
    </div>
    <div class="setting1">
      <tiny-button class="settingbtn" type="primary" @click="setting"
        >确定</tiny-button
      >
      <tiny-button class="settingbtn" type="primary" @click="downloadimg"
        >批量下载</tiny-button
      >
    </div>
  </div>

  <right-menu>
    <template #default>
      <img
        class="rightmenu-img"
        :src="'/icon/setting.png'"
        @click="showsetting"
      />
    </template>
  </right-menu>
</template>

<style scoped>
.new-loading .tiny-loading__spinner .tiny-loading__text {
  color: white;
}

.setting1 {
  display: flex;
  justify-content: center;
  width: 100vw;
  box-sizing: border-box;
  padding: 20px;
}
.settingbtn {
  display: flex;
  width: 100px;
  justify-content: center;
  border: none;
}
.groupbtn {
  display: flex;
  width: 100vw;
  box-sizing: border-box;
  padding: 10px;
}

.groupbtntitle {
  width: 100px;
  height: 28px;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 12px;
  background: white;
  margin: 0 30px;
}
.setting {
  position: absolute;
  z-index: 200;
  top: 0px;
  left: 0px;
  width: 100vw;
  height: 100vh;
  background: rgba(230, 230, 230, 0.5);
  border-radius: 15px;
  display: flex;
  justify-content: start;
  align-content: center;
  flex-wrap: wrap;
  box-sizing: border-box;
  padding: 50px;
}

.btn {
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

.btn:hover {
  box-shadow: 0px 2px 5px 2px rgba(23, 23, 23, 1);
}
::-webkit-scrollbar {
  display: none;
}

.rightmenu-img {
  width: 25x;
  height: 25px;
  display: flex;
}

.wallpapers-div {
  width: 100vw;
  height: 100vh;
  overflow: hidden;
  overflow-y: scroll;
  background-color: white;
  text-align: center;
}
.wallpapers {
  column-count: 4;
  column-gap: 5px;
  height: auto;
  margin: 10px 10px;
}
.imgbtn {
  position: absolute;
  z-index: 100;
  top: 0px;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  opacity: 0;
  align-items: center;
  justify-content: space-around;


}
.imgbtn:hover{
    opacity: 1;

}
</style>
