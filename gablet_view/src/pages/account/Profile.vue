<script setup lang="ts">
import useAuthStore from '@/stores/useAuthStore';
import { ref } from 'vue';
import api from '@/api/api';
import { getErrorMessage } from '@/utils/errors';
import { useTranslation } from 'i18next-vue';

const user = ref('Loading...');
const {t} = useTranslation();

const loadProfile = async () => {
    const profile = await api.profile.profile();
    user.value = `${profile.username ?? getErrorMessage(profile.error, t)}`;
}

loadProfile();

</script>

<template>
    <h1>Profile</h1>
    <h3>Username: {{ user }}</h3>
</template>