<template>
    <div class="flex relative mt-8">
        <i
            class="flex place-items-center place-content-center w-8 h-8 text-zinc-100 border rounded-lg border-zinc-100 before:border-l before:absolute before:left-0 before:bottom-1 before:h-16 top-16">
            <slot name="icon"></slot>
        </i>
        <div class="flex-1 ml-4">
            <h3 class="text-white text-xl mb-2 font-medium">
                <slot name="heading"></slot>
            </h3>
            <slot></slot>
        </div>
    </div>
</template>
