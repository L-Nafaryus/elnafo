import { createApp } from 'vue';
import App from '@/App.vue';
import router from '@/router';

import '@/assets/style.css';

import Me from '@/views/Me.vue'
import Home from '@/views/Home.vue'
createApp(App)
    .use(router)
    .mount('#app');

