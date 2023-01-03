import {defineStore} from "pinia";
import {ref} from "vue";
import {SidebarMenuItem} from "./type";

export declare type SidebarMenuStore = {
  menus: SidebarMenuItem[]
  bottomMenus: SidebarMenuItem[]
}

export const menus = ref<SidebarMenuItem[]>()

export const setMenus = (newMenus: SidebarMenuItem[]) => {
    menus.value = newMenus
}

export const bottomMenus = ref<SidebarMenuItem[]>([
  {
    id: 'help',
    name: 'Help',
    icon: 'fe fe-help-circle',
    routePath: '/help',
  },
  {
    id: 'settings',
    name: 'Settings',
    icon: 'fe fe-sliders',
    routePath: '/settings',
  }
])
