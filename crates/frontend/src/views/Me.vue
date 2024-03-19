<script setup lang="ts">
import Base from '@/views/Base.vue';
import { ref, onMounted } from 'vue';
import axios from 'axios';

const email = ref(null);
const name = ref(null);
const is_admin = ref(null);
const errorMessage = ref(null);

onMounted(async () => {
    const asd = await fetch(import.meta.hot ? "http://localhost:54600/api/v1/me" : "/api/v1/me", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
        mode: "cors",
    })
        .then(async response => {
            const isJson = response.headers.get('content-type')?.includes('application/json');
            const data = isJson && await response.json();

            if (!response.ok) {
                const error = (data && data.message) || response.status;
                return Promise.reject(error);
            }

            name.value = data.data.user.name;
            email.value = data.data.user.email;
            is_admin.value = data.data.user.is_admin;
        })
        .catch(error => {
            errorMessage.value = error;
            console.error("Error occured:", error);
        });
})
</script>

<template>
    <Base>
    <p v-if="errorMessage" class="text-center pt-3 pb-3 bg-orange-900 rounded border border-orange-700">{{
        errorMessage }}</p>
    <p>{{ name }}</p>
    <p>{{ email }}</p>
    <p>{{ is_admin }}</p>
    </Base>
</template>
