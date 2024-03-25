import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: "/", component: () => import("@/views/Home.vue") },
        { path: "/user/login", component: () => import("@/views/SignIn.vue") },
        { path: "/:user", name: "User", component: () => import("@/views/User.vue") },
        { path: "/:pathMatch(.*)*", component: () => import("@/views/Error.vue") }
    ]
});

export default router;
