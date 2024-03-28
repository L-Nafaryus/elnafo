import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: "/", name: "Home", component: () => import("@/views/Home.vue") },

        { path: "/user/login", name: "SignIn", component: () => import("@/views/user/SignIn.vue") },
        { path: "/user/register", name: "SignUp", component: () => import("@/views/user/SignUp.vue") },
        {
            path: "/user/preferencies", name: "Preferencies", redirect: { name: "Preferencies-Profile" }, component: () => import("@/views/user/Preferencies.vue"), children: [
                { path: "profile", name: "Preferencies-Profile", component: () => import("@/views/user/preferencies/Profile.vue") },
                { path: "account", name: "Preferencies-Account", component: () => import("@/views/user/preferencies/Account.vue") },
            ]
        },

        { path: "/:user", name: "Profile", component: () => import("@/views/user/Profile.vue") },

        { path: "/admin/settings", name: "Settings", component: () => import("@/views/admin/Settings.vue") },

        { path: "/:pathMatch(.*)*", name: "NotFound", component: () => import("@/views/error/NotFound.vue") }
    ]
});

export default router;
