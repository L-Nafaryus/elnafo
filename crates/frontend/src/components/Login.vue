<script setup lang="ts">
import { ref } from 'vue';
import router from '@/router';

const email = defineModel("email");
const password = defineModel("password");
const errorMessage = ref(null);

async function login() {
    await fetch(import.meta.hot ? "http://localhost:54600/api/v1/login_user" : "/api/v1/login_user", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        body: JSON.stringify({ email: email.value, password: password.value })
    })
        .then(async response => {
            //const isJson = response.headers.get('content-type')?.includes('application/json');
            const data = await response.json();

            if (!response.ok) {
                const error = (data && data.message) || response.status;
                return Promise.reject(error);
            }

            router.push({ path: '/me' });
        })
        .catch(error => {
            errorMessage.value = error;
            console.error(error);
        });
};
</script>

<template>
    <div class="ml-auto mr-auto w-1/2 pt-5 pb-5">
        <h4 class="text-center pt-5 pb-5 border-b border-zinc-500">Sign In</h4>
        <form @submit.prevent class="m-auto pt-5 pb-5">
            <div class="mb-5 ml-auto mr-auto">
                <label for="email" class="text-right w-64 inline-block mr-5">Email Address</label>
                <input v-model="email" type="email" placeholder="" name="email" required
                    class="w-1/2 bg-zinc-800 pl-3 pr-3 pt-2 pb-2 outline-none rounded border border-zinc-500 hover:border-zinc-400 focus:border-green-800">
            </div>
            <div class="mb-5 ml-auto mr-auto">
                <label for="password" class="text-right w-64 inline-block mr-5">Password</label>
                <input v-model="password" placeholder="" type="password" name="password" required
                    class="w-1/2 bg-zinc-800 pl-3 pr-3 pt-2 pb-2 outline-none rounded border border-zinc-500 hover:border-zinc-400 focus:border-green-800">
            </div>
            <div class="mb-5 ml-auto mr-auto">
                <label class="text-right w-64 inline-block mr-5"></label>
                <button @click="login" class="rounded bg-zinc-500 hover:bg-zinc-400 pb-2 pt-2 pl-5 pr-5">Sign
                    In</button>
            </div>
        </form>
        <p v-if="errorMessage" class="text-center pt-3 pb-3 bg-orange-900 rounded border border-orange-700">{{
                    errorMessage }}</p>
    </div>
</template>
