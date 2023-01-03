import {onBeforeUnmount, onMounted} from "vue";
import {UnlistenFn} from "@tauri-apps/api/event";


export const useUnListen = (fn: () => Promise<UnlistenFn>) => {
    let unListen: UnlistenFn | undefined
    onMounted(async () => {
        unListen = await fn()
    })

    onBeforeUnmount(() => {
        unListen?.()
        unListen = undefined
    })
}
