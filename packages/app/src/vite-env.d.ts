/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare interface Shortcut {
  keystroke: string;
  mods: string[];
  title: string;
}

declare interface ModState {
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
  shift: boolean;
}
