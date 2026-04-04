/*
 * TokenGo - A lightweight TOTP password manager
 * Copyright (C) 2024 ant-cave <ANTmmmmm@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

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