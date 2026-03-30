import { createRouter, createWebHashHistory } from 'vue-router'
import Auth from '../views/Auth.vue'
import Home from '../views/Home.vue'
import AddSecret from '../views/AddSecret.vue'
import ImportSecret from '../views/ImportSecret.vue'

// 路由配置
const routes = [
  { path: '/', redirect: '/auth' },
  { path: '/auth', component: Auth },
  { path: '/home', component: Home },
  { path: '/add', component: AddSecret },
  { path: '/import', component: ImportSecret },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
