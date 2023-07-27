<script setup lang="ts">
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import * as yup from 'yup';
import useAuthStore from '@/stores/useAuthStore';
import api from '@/api/api';
import { useTranslation } from 'i18next-vue';
import { getErrorMessage } from '@/utils/errors';
import type { LoginResponse } from '@/api/auth';

const emit = defineEmits<{
    login: [user: LoginResponse]
}>();

const { t } = useTranslation();
const auth = useAuthStore();

const apiError = ref('');
const loggingIn = ref(false);

const schema = yup.object({
    username: yup.string().required().label(t('signin.usernameOrEmail')),
    password: yup.string().required().min(8).label(t("signin.password"))
});

const formState = useForm({ validationSchema: schema });

const { defineComponentBinds, handleSubmit, errors } = formState;

const username = defineComponentBinds('username');
const password = defineComponentBinds('password');

const onSubmit = handleSubmit(async (values) => {
    loggingIn.value = true;
    apiError.value = '';
    try {
        let result = await api.auth.login({ username: values.username, password: values.password });
        if (result.error) {
            apiError.value = getErrorMessage(result.error, t);
            return;
        }

        if (!auth.setLogin(result)) {
            apiError.value = t('signin.signInError', { error: t('signin.invalidResponse') });
        }

        emit('login', result);
    } catch(err) {
        apiError.value = getErrorMessage(err, t);
        return;
    } finally {
        loggingIn.value = false;
    }
});

</script>

<template>
    <form class="mt-4" @submit.prevent="onSubmit">
        <div class="gablet-signin-input">
            <div class="p-float-label">
                <InputText 
                    type="text" 
                    v-bind="username" 
                    id="signInUsername"
                    class="gablet-signin-input-text"
                    :class="{ 'p-invalid': errors.username }" />
                <label for="signInUsername">{{ t('signin.usernameOrEmail') }}</label>
            </div>
            <small id="signInUsername-help" class="p-error">
                {{ errors.username }}
            </small>
        </div>
        <div class="gablet-signin-input">
            <span class="p-float-label">
                <Password 
                    v-bind="password"
                    :feedback="false"
                    toggleMask
                    inputId="signInPassword"
                    class="gablet-signin-input-text"
                    :class="{ 'p-invalid': errors.password }" />
                <label for="signInPassword">Password</label>
            </span>
            <small id="signInPassword-help" class="p-error">
                {{ errors.password }}
            </small>
        </div>
        <Button class="gablet-signin-button" type="submit" label="Submit" :disabled="loggingIn" />
        <small v-if="apiError" class="p-error">
            {{ apiError }}
        </small>
    </form>
</template>

<style scoped>

.gablet-signin-input {
    margin-bottom: 1.5rem;
}

.gablet-signin-input-text {
    width: 100%;
}

.gablet-signin-button {
    width: 100%;
}

</style>