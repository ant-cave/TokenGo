import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import '@primer/css/dist/marketing.css';
import '@primer/css/dist/primer.css';
import './styles/github-cards.css';

// Element Plus Icons
import * as ElementPlusIconsVue from '@element-plus/icons-vue';

const app = createApp(App);

// 全局注册 Element Plus 图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

app.use(router);
app.mount("#app");

document.addEventListener('contextmenu', (e) => {
    e.preventDefault();
    // TODO: 如果需要调试可以改成条件判断，比如按住Shift时允许右键
});