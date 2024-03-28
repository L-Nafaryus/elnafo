<script setup lang="ts">
import Base from "@/views/Base.vue";

import { ref, onMounted, watch, getCurrentInstance } from "vue";

import router from "@/router";
import User from "@/services/user";
import { useUserStore } from "@/stores/user";
import { usePreferenciesStore } from "@/stores/preferencies.ts";

const login = defineModel("login");
const name = defineModel("name");
const email = defineModel("email");

const error = ref(null);
const userStore = useUserStore();
const preferenciesStore = usePreferenciesStore();

onMounted(async () => {
    preferenciesStore.current_tab = 0;

    !userStore.login ? router.push({ name: "SignIn" }) : await User.current()
        .then(async response => {
            error.value = null;

            if (response.status != 200) {
                return Promise.reject(response.data && response.data.message || response.status);
            };

            login.value = response.data.user.login;
            name.value = response.data.user.name;
            email.value = response.data.user.email;
        })
        .catch(e => {
            error.value = e;
            console.log(`${e.name}[${e.code}]: ${e.message}`);
        })
});
</script>

<template>
    <div class="border rounded border-zinc-500 w-full flex-col">
        <h1 class="pl-5 pr-5 pt-2 pb-2">Profile Info</h1>
        <div class="border-t border-zinc-500 p-5">
            <form @submit.prevent class="">
                <div>
                    <label class="block mb-2" for="login">Login</label>
                    <input v-model="login" name="login"
                        class="w-full bg-zinc-800 pl-3 pr-3 pt-2 pb-2 mb-4 outline-none rounded border border-zinc-500 hover:border-zinc-400 focus:border-green-800">
                </div>
                <div>
                    <label class="block mb-2" for="name">Username</label>
                    <input v-model="name" name="name"
                        class="w-full bg-zinc-800 pl-3 pr-3 pt-2 pb-2 mb-4 outline-none rounded border border-zinc-500 hover:border-zinc-400 focus:border-green-800">
                </div>
                <div>
                    <label class="block mb-2 " for="email">Email</label>
                    <input v-model="email" email="email" disabled
                        class="w-full bg-zinc-800 pl-3 pr-3 pt-2 pb-2 mb-4 outline-none rounded border border-zinc-500 hover:border-zinc-400 focus:border-green-800">
                </div>
                <div class="border-t border-zinc-500 ml-0 mr-0 mt-3 mb-3"></div>
                <button
                    class="rounded bg-zinc-500 hover:bg-zinc-400 pb-2 pt-2 pl-5 pr-5 ml-auto mr-0 block">Update</button>
            </form>
        </div>
    </div>
</template>
