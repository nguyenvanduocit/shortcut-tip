import {Router} from "vue-router";
import {App} from "vue";

declare interface Option {
    router: Router;
}

export default {
    install(app: App, options: Option) {
        options.router.addRoute({
            path: "/projects",
            name: "projectList",
            components: {
                default: () => import("./views/ViewProjects.vue")
            }
        });
    }
}
