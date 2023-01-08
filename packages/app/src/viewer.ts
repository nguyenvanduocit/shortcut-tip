import { createApp } from "vue";
import AppViewer from "./AppViewer.vue";
import "./styles/viewer.styl"

const app =createApp(AppViewer)
app.mount("#app");

