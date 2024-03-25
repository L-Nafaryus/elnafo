<script setup lang="ts">
import Base from '@/views/Base.vue';
import Error from "@/components/Error.vue";
import { ref, onMounted, watch } from 'vue';
import { onBeforeRouteUpdate, useRoute } from 'vue-router'
import User from "@/services/user";

const route = useRoute();
const name = ref(null);
const error = ref(null);

onMounted(async () => {
    await User.get(route.params.user)
        .then(async response => {
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
});

watch(() => route.params.user, async (to, from) => {
    await User.get(route.params.user)
        .then(async response => {
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
});
</script>

<template>
    <Base>
    <div v-if="error">
        <Error>{{ error }}</Error>
    </div>
    <p v-else>{{ name }}</p>
    </Base>
</template>
