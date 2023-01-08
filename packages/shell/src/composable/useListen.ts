import {onBeforeUnmount, onMounted} from "vue";
import {EventCallback, listen, UnlistenFn} from "@tauri-apps/api/event";


export const useListen = <T>(topic: string, fn: EventCallback<T>) => {
    let unListen: UnlistenFn | undefined
    onMounted(async () => {
        unListen = await listen<T>(topic, fn)
    })

    onBeforeUnmount(() => {
        unListen?.()
        unListen = undefined
    })
}
