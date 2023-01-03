import {App} from "vue";
import {loadStyle} from "./src/styles";

export * from "./src/sidebar"
export * from "./src/util"
export * from './src/menubar'
export * from './src/styles'
export * from './src/views'
export * from './src/composable'
export * from './src/modal'


export default {
    install: (app: App) => {
        loadStyle()
    }
}
