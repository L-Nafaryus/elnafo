import { createRouter, createWebHistory } from 'vue-router';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: () => import('./views/Home.vue') },
        { path: '/user/login', component: () => import('./views/SignIn.vue') },
        { path: '/me', component: () => import('./views/Me.vue') }
    ]
});

export default router;
