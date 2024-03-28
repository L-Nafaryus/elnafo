<script setup lang="ts">
import Meerkat from "@/components/icons/Meerkat.vue";
import NavBar from "@/components/NavBar.vue";
import DropdownMenu from "@/components/DropdownMenu.vue";

import { ref, onMounted } from "vue";

import router from "@/router";
import User from "@/services/user";
import { useUserStore } from "@/stores/user";

const user = ref(null);
const userStore = useUserStore();
const error = ref<string>(null);

onMounted(async () => {
    await User.current()
        .then(async response => {
            if (response.status != 200) {
                return Promise.reject(response.data && response.data.message || response.status);
            };
            if (response.data.hasOwnProperty("user")) {
                userStore.login = response.data.user.login;
            };
        })
        .catch(e => {
            console.log(`${e.name}[${e.code}]: ${e.message}`);
        });
});

async function user_logout() {
    await User.logout()
        .then(async response => {
            error.value = null;

            if (response.status != 200) {
                return Promise.reject(response.data && response.data.message || response.status);
            };

            userStore.login = null;
            router.push({ path: "/" });
        })
        .catch(e => {
            console.error("Error occured:", e);
        });
}

</script>

<template>
    <div class="flex-grow pb-20">
        <NavBar>
            <template #left>
                <Meerkat />
            </template>
            <template #right>
                <DropdownMenu v-if="userStore.login">
                    <template #button>
                        <span
                            class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600 cursor-pointer">{{
                    userStore.login }}</span>
                    </template>
                    <template #content>
                        <div
                            class="absolute flex flex-col left-auto right-0 mt-4 bg-zinc-700 border rounded border-zinc-500 mr-3">
                            <RouterLink :to="{ name: 'Profile', params: { user: userStore.login } }"
                                class="flex min-w-7 pl-5 pr-5 pt-1 pb-1 hover:bg-zinc-600">
                                Profile</RouterLink>
                            <RouterLink :to="{ name: 'Preferencies' }"
                                class="flex min-w-7 pl-5 pr-5 pt-1 pb-1 hover:bg-zinc-600">
                                Preferencies</RouterLink>
                            <div class="border-t border-zinc-500 ml-0 mr-0"></div>
                            <RouterLink :to="{ name: 'Settings' }"
                                class="flex min-w-7 pl-5 pr-5 pt-1 pb-1 hover:bg-zinc-600">
                                Settings</RouterLink>
                            <div class="border-t border-zinc-500 ml-0 mr-0"></div>
                            <div @click="user_logout"
                                class="flex min-w-7 pl-5 pr-5 pt-1 pb-1 hover:bg-zinc-600 cursor-pointer">
                                Sign Out</div>
                        </div>
                    </template>
                </DropdownMenu>

                <RouterLink v-if="!userStore.login"
                    class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600" to="/user/login">
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
