<script setup>
import { onMounted, defineProps, defineEmits } from 'vue'
import { initFlowbite } from 'flowbite'
defineProps({
    barState: Boolean
})
defineEmits(["switchSideBarState"])

// initialize components based on data attribute selectors
onMounted(() => {
    initFlowbite();
})
</script>

<template>

    <aside id="logo-sidebar"
        class="fixed top-36 left-0 z-40 w-64 h-screen transition-transform -translate-x-full bg-white border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700"
        :class="barState === true ? 'translate-x-0' : '-translate-x-full'" aria-label="Sidebar">
        <div class="h-full px-3 pb-4 overflow-y-auto bg-white dark:bg-gray-800">
            <ul class="space-y-2 font-medium">
                <slot name="element"></slot>
            </ul>
        </div>
    </aside>

    <div>
        <button @click="$emit('switchSideBarState')" class="dark:bg-slate-700 rounded-lg fixed top-2/4 z-40"
            :class="barState === false ? 'rotate-180 left-0' : 'left-52'">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                stroke="currentColor" class="size-6">
                <path stroke-linecap="round" stroke-linejoin="round"
                    d="M8.25 9V5.25A2.25 2.25 0 0 1 10.5 3h6a2.25 2.25 0 0 1 2.25 2.25v13.5A2.25 2.25 0 0 1 16.5 21h-6a2.25 2.25 0 0 1-2.25-2.25V15m-3 0-3-3m0 0 3-3m-3 3H15" />
            </svg>
        </button>
    </div>

</template>