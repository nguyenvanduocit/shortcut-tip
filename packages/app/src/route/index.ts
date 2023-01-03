import {createRouter, createWebHistory} from "vue-router";
import {AppMenuBarDefault } from '@aiocean/shell'
import {View404} from '@aiocean/shell'
const routes = [
  {
    path: '/',
    redirect: '/projects',
  },
  {
    path: "/help",
    components: {
      default: () => import('../views/ViewHelp.vue'),
      menuBar: () => import('../components/TopBarHelp.vue'),
    }
  },
  {
    path: "/about",
    components: {
      default: () => import('../views/ViewAbout.vue'),
      menuBar: AppMenuBarDefault
    }
  },
  {
    path: "/settings",
    components: {
      default: () => import('../views/ViewSetting.vue'),
      menuBar: AppMenuBarDefault
    }
  },
  {
    path: "/:pathMatch(.*)*",
    components: {
      default: View404,
      menuBar: AppMenuBarDefault
    }
  }
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
