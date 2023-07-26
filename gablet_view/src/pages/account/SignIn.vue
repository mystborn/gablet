<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';
import SignInForm from '@/components/SignInForm.vue';
import type { LoginResponse } from '@/api/auth';
import { useTranslation } from 'i18next-vue';
import Header from '@/components/Header.vue';
import Footer from '@/components/Footer.vue';
import { devLog } from '@/utils/errors';

const router = useRouter();
const route = useRoute();
const { t } = useTranslation();

const onLogin = (response: LoginResponse) => {
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
}

</script>

<template>
    <div class="flex-grow-1 flex justify-content-center align-items-center">
        <Card class="gablet-signin-container">
            <template #title>{{ t('signin.signIn') }}</template>
            <template #content>
                <SignInForm @login="onLogin" />
            </template>
        </Card>
    </div>
</template>

<style scoped>

.gablet-signin-container {
    padding: 1.5rem;
    height: fit-content;
}

</style>