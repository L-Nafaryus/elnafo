<script setup lang="ts">
const email = defineModel("email")
const password = defineModel("password")

async function login() {
    const response = await fetch(
        "http://0.0.0.0:54600/api/v1/login_user",
        {
            method: "POST",
            headers: {
                //Accept: 'application/json',
                "Content-Type": "application/json",
                //"Access-Control-Allow-Origin": "http://0.0.0.0"
            },
            credentials: "include",
            mode: "cors",
            body: JSON.stringify({ email: email.value, password: password.value })
        }
    );

    let { status, token } = await response.json();
    console.log(status);
}
</script>

<template>
    <form @submit.prevent>
        <input v-model="email" type="email" placeholder="Email" required>
        <input v-model="password" placeholder="password" type="password" required>
        <button @click="login">Log in</button>
    </form>
</template>

<!--
<script setup lang="ts">
import { ref } from "vue";
const emit = defineEmits<{
  (e: "create", payload: { body: string; title: string }): void;
}>();
const title = ref("");
const body = ref("");
const handleSubmit = () => {
  emit("create", { body, title });
};
</script>
<template>
  <form @submit="handleSubmit">
    <input v-model="title" />
    <textarea v-model="body"></textarea>
    <button>Create Post</button>
  </form>
</template>

-->
