<script setup lang="ts">
defineProps<{
    msg: string
}>()
</script>

<template>
    <div class="text-center">
        <h1 class="text-green-500 relative text-4xl">{{ msg }}</h1>
        <h3 class="text-xl">
            You’ve successfully created a project with
            <a href="https://vitejs.dev/" target="_blank" rel="noopener">Vite</a> +
            <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a>.
        </h3>
    </div>
</template>
