import {ref} from "vue";
import {createGlobalState} from "@vueuse/core";

export const useStore = createGlobalState(() => {
    const projects = ref<Record<string, Shortcut>>({})

    const loadProjects = async () => {
    }

    const addProject = async (keystroke: string, title: string) => {
    }

    const removeProject = async (keystroke: string) => {
        console.log(keystroke)
        const projectId = btoa(keystroke);
        delete projects.value[projectId]
    }

    const clearProjects = async () => {
        projects.value = {}
    }

    const getProject = async (projectId: string) => {
        if (!projects.value[projectId]) {
            await loadProjects()
        }

        return projects.value[projectId]
    }

    // init
    loadProjects().then(r => console.log('projects loaded'));

    return {
        projects,
        addProject,
        removeProject,
        loadProjects,
        clearProjects,
        getProject
    }
})
