<script setup lang="ts">
import Base from "@/views/Base.vue";
import Error from "@/components/error/Error.vue";

import { ref, onMounted, watch, getCurrentInstance } from "vue";
import { onBeforeRouteUpdate, useRoute } from "vue-router"

import User from "@/services/user";
import { useUserStore } from "@/stores/user";

const route = useRoute();
const name = ref<string>(null);
const userStore = useUserStore();
const error = ref<string>(null);

async function user_profile(login: string) {
    await User.get(login)
        .then(async response => {
            error.value = null;

            if (response.status != 200) {
                return Promise.reject(response.data && response.data.message || response.status);
            };

            if (response.data.hasOwnProperty("user")) {
                name.value = response.data.user.name;
            } else {
                error.value = "404 Not Found";
            };

        })
        .catch(e => {
            console.error("Error occured:", e);
        });
};

onMounted(async () => {
    await user_profile(route.params.user);
});

watch(route, async (to, from) => {
    await user_profile(to.params.user);
});
</script>

<template>
    <Base>
    <div class="ml-auto mr-auto w-1/2 pt-5 pb-5">
        <Error v-if="error">{{ error }}</Error>
        <p v-else>{{ name }}</p>
    </div>
    </Base>
</template>
