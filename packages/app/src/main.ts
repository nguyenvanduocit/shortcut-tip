import { createApp } from "vue";
import App from "./App.vue";
import {router} from "./route";
import projectListModule from './module/projectList'
import shell from '@aiocean/shell'
import './styles/main.styl'

const app =createApp(App)
app.use(shell)
app.use(projectListModule, {router})
app.use(router)
app.mount("#app");

