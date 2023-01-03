import {ref} from "vue";

export const useModal = ()=>{
    const isShow = ref(false)
    const show = ()=>{
        isShow.value = true
    }
    const close = ()=>{
        isShow.value = false
    }
    return {
        isShow,
        show,
        close
    }
}
