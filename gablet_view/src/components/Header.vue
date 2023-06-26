<script setup lang="ts">

import { router } from "@/router";
import type { MenuItem } from "primevue/menuitem";
import { ref } from "vue";

const hovering = ref('');
const searchText = ref('');

type NavigationItem = {
    id: string,
    label: string,
    route?: string
}

const navigation = [
    {
        id: 'gablet-header-popular',
        label: 'Popular',
    },
    {
        id: 'gablet-header-genres',
        label: 'Genres',
    },
    {
        id: 'gablet-header-new',
        label: 'New',
    }
] as NavigationItem[];

const iconMenu: MenuItem[] = [
    {
        icon: 'pi pi-search',
    },
    {
        icon: 'pi pi-bell',
    },
    {
        icon: 'pi pi-user'
    }
]

const setHovering = (e: MouseEvent) => {
    hovering.value = e.target.id || e.target.parentNode.id;
}

const resetHovering = () => {
    hovering.value = '';
}

const getSeverity = (index: number) => {
    if(hovering.value === navigation[index].id) {
        return '';
    }

    return 'secondary';
}

</script>

<template>
    <Toolbar class="gablet-toolbar flex-grow-1 gablet-content py-0">
        <template #start>
            <span class="flex">
                <Button 
                    id="gablet-header-home" class="gablet-title text-4xl" label="Gablet" text
                    @click="$router.push({ name: 'Home' })" />

                <template v-for="(item, index) in navigation">
                    <Button 
                        :id="item.id" class="gablet-header-link" :label="item.label" text
                        :severity="getSeverity(index)"
                        @mouseover="setHovering"
                        @mouseleave="resetHovering"
                        @click="$router.push({ name: item.route ?? item.label })" />
                </template>
            </span>
        </template>
        <template #end>
            <span class="p-input-icon-right">
                <i class="pi pi-search" />
                <InputText v-model="searchText" placeholder="Search..." />
            </span>
            <Button icon="pi pi-bell" class="gablet-header-icon" severity="secondary" aria-label="Notifications" text />
            <Button icon="pi pi-user" class="gablet-header-icon" severity="secondary" aria-label="Profile" text />
        </template>
    </Toolbar>
</template>

<style scoped>

.gablet-toolbar {
    position: sticky;
    top: 0;
    z-index: 1000;
}

.gablet-title {
    background-color: transparent !important;
    box-shadow: none;
}

.gablet-header-link {
    background-color: transparent !important;
    box-shadow: none !important;
}

.gablet-header-icon {
    box-shadow: none !important;
}

</style>