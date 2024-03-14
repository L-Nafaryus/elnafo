<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'
import TheWelcome from './components/TheWelcome.vue'
import Login from './components/Login.vue'

import { ref, watchEffect } from 'vue'
const count = ref(0)

//const url = `http://127.0.0.1:8000/api/hello`
//const response = ref(null)

//watchEffect(async () => {
//    response.value = await (await fetch(url)).json()
//})
</script>

<template>
    <!--<header class="flex flex-wrap place-items-center">
        <img alt="Vue logo" class="block ml-auto mr-auto w-32 h-32" src="./assets/logo.svg" />

        <div class="wrapper">
            <HelloWorld msg="You did it!" />
        </div>

        <button class="font-bold" @click="count++">Count is: {{ count }}</button>
    </header>

    <main>
        <TheWelcome />

        <p>{{ response }}</p>
    </main>-->
    <main>
        <Login />
    </main>
</template>
