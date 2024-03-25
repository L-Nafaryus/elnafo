<script setup lang="ts">
import Meerkat from '@/components/icons/Meerkat.vue';
import NavBar from '@/components/NavBar.vue';
import User from "@/services/user";
import { ref, onMounted } from 'vue';

const user = ref(null);

onMounted(async () => {
    await User.current()
        .then(async response => {
            if (response.status != 200) {
                return Promise.reject(response.data && response.data.message || response.status);
            };
            if (response.data.hasOwnProperty("user")) {
                user.value = response.data.user;
            };
        })
        .catch(e => {
            console.error("Error occured:", e);
        });
});
</script>

<template>
    <div class="flex-grow pb-20">
        <NavBar>
            <template #left>
                <Meerkat />
            </template>
            <template #right>
                <RouterLink v-if="user" class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600"
                    :to="{ name: 'User', params: { user: user.login } }">{{ user.name }}</RouterLink>
                <RouterLink v-if="!user" class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600"
                    to="/user/login">
                    Sign In</RouterLink>
            </template>
        </NavBar>

        <main>
            <slot></slot>
        </main>
    </div>
    <footer class="flex justify-between pb-2 pt-2 pl-5 pr-5 bg-zinc-800 border-t border-t-zinc-500">
        <a href="/">Made with glove</a>
        <a href="/api/v1">API</a>
    </footer>

</template>
