<script setup lang="ts">
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import * as yup from 'yup';
import api from '@/api/api';
import useAuthStore from '@/stores/useAuthStore';
import { useTranslation } from 'i18next-vue';
import { getErrorMessage } from '@/utils/errors';
import { useRoute, useRouter } from 'vue-router';
import { devLog } from '@/utils/errors';
import RegisterForm from '@/components/RegisterForm.vue';
import type { LoginResponse } from '@/api/auth';

const { t } = useTranslation();
const route = useRoute();
const router = useRouter();

const onRegister = (response: LoginResponse) => {
    const redirect = route.query.link?.toString();
    if(!redirect) {
        router.replace('/');
        return;
    }

    const destination = decodeURI(redirect);

    if (destination.startsWith('/')) {
        try {
            router.replace(destination);
            return;
        } catch(err) {
            devLog(`Failed to redirect to ${destination}. Error: `, err);
        }
    }

    window.location.href = decodeURI(redirect);
};

</script>

<template>
    <div class="flex-grow-1 flex justify-content-center align-items-center">
        <Card class="gablet-register-container">
            <template #title>{{ t('signin.register') }}</template>
            <template #content>
                <RegisterForm @register="onRegister" />
            </template>
        </Card>
    </div>
</template>

<style scoped>

.gablet-register-container {
    padding: 1.5rem;
    height: fit-content;
}

</style>