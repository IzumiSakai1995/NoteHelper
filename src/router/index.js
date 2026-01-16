import { createRouter, createWebHistory } from 'vue-router'
import Novels from '../views/Novels.vue'
import NovelDetail from '../views/NovelDetail.vue'

const routes = [
  { path: '/', component: Novels },
  { path: '/novel/:id', name: 'NovelDetail', component: NovelDetail, props: true }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
