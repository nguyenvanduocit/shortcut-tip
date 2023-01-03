import {ref} from "vue";
import {createGlobalState} from "@vueuse/core";
import {Store} from "tauri-plugin-store-api";


export const useStore = createGlobalState(() => {
    const store = new Store('.shortcut.dat');
    const projects = ref<Record<string, Shortcut>>({})

    const loadProjects = async () => {

        const entry = await store.entries<Shortcut>()
        entry.forEach(value => {
            console.log(value)
            projects.value[value[0]] = value[1]
        })
    }

    const addProject = async (keystroke: string, title: string) => {
        // base6
        const projectId = btoa(keystroke);

        projects.value[projectId] = {
            keystroke: keystroke,
            title: title
        };

        await store.set(projectId, projects.value[projectId])
    }

    const removeProject = async (keystroke: string) => {
        console.log(keystroke)
        const projectId = btoa(keystroke);
        await store.delete(projectId)
        delete projects.value[projectId]
    }

    const clearProjects = async () => {
        await store.clear()
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
