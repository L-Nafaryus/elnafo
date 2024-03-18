<script setup lang="ts">
import Meerkat from '@/components/icons/Meerkat.vue';
import NavBar from '@/components/NavBar.vue';

function next() {
    window.location.pathname = "/me";
};
</script>

<template>
    <div class="flex-grow pb-20">
        <NavBar>
            <template #left>
                <Meerkat />
            </template>
            <template #right>
                <RouterLink class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600" to="/user/login">
                    Sign In</RouterLink>
                <a class="flex min-w-9 min-h-9 pt-1 pb-1 pl-3 pr-3 rounded hover:bg-zinc-600" href="/user/register">Sign
                    up</a>
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
