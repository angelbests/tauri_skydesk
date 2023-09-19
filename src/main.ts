import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import router from "./router";
import TinyVue from '@opentiny/vue'
const app = createApp(App)
app.use(router);
app.use(TinyVue);
app.mount("#app");
