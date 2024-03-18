import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: () => import('./views/Home.vue') },
        { path: '/user/login', component: () => import('./views/SignIn.vue') }
    ]
});

export default router;
