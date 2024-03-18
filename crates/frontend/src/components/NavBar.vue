<template>
    <nav class="flex justify-between items-center m-0 pl-3 pr-3 bg-zinc-800 border-b border-b-zinc-500">
        <div class="items-center m-0 flex">
            <slot name="left"></slot>
        </div>
        <div class="items-center m-0 flex">
            <slot name="right"></slot>
        </div>
    </nav>
</template>
