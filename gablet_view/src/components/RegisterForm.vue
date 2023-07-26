<script setup lang="ts">
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import * as yup from 'yup';
import api from '@/api/api';
import useAuthStore from '@/stores/useAuthStore';
import { useTranslation } from 'i18next-vue';
import { getErrorMessage } from '@/utils/errors';
import type { LoginResponse } from '@/api/auth';

const emit = defineEmits<{
    register: [user: LoginResponse]
}>();

const schema = yup.object({
    username: yup.string().required().label("Username"),
    email: yup.string().email().required().label("Email"),
    password: yup.string().required().min(8).label("Password")
});

const authStore = useAuthStore();
const { t } = useTranslation();

const formState = useForm({ validationSchema: schema });
const { defineComponentBinds, handleSubmit, errors } = formState;

const username = defineComponentBinds('username');
const email = defineComponentBinds('email');
const password = defineComponentBinds('password');

const apiError = ref('');
const registering = ref(false);

const onSubmit = handleSubmit(async (values) => {
    registering.value = true;
    try {
        let result = await api.auth.register({
            username: values.username,
            email: values.email,
            password: values.password
        });

        if (result.error) {
            apiError.value = t('signin.registerError', { error: getErrorMessage(result.error, t) });
            return;
        }

        authStore.setLogin(result);

        emit('register', result);
    } catch(err) {
        apiError.value = t('signin.registerError', { error: getErrorMessage(err, t) });
    } finally {
        registering.value = false;
    }
});

</script>

<template>
    <div class="centered card flex justify-content-center">
        <Card class="gablet-signin-container">
            <template #content>
                <form @submit.prevent="onSubmit">
                    <div class="gablet-signin-input">
                        <div class="p-float-label">
                            <InputText 
                                type="text" 
                                v-bind="username" 
                                id="registerUsername"
                                class="gablet-signin-input-text"
                                :class="{ 'p-invalid': errors.username }" />
                            <label for="registerUsername">Username</label>
                        </div>
                        <small id="registerUsername-help" class="p-error">
                            {{ errors.username }}
                        </small>
                    </div>
                    <div class="gablet-signin-input">
                        <span class="p-float-label">
                            <InputText 
                                type="text" 
                                v-bind="email" 
                                id="registerEmail"
                                class="gablet-signin-input-text"
                                :class="{ 'p-invalid': errors.email }" />
                            <label for="registerEmail">Email</label>
                        </span>
                        <small id="registerEmail-help" class="p-error">
                            {{ errors.email }}
                        </small>
                    </div>
                    <div class="gablet-signin-input">
                        <span class="p-float-label">
                            <Password 
                                v-bind="password" 
                                toggleMask
                                inputId="registerPassword"
                                class="gablet-signin-input-text"
                                :class="{ 'p-invalid': errors.password }" />
                            <label for="registerPassword">Password</label>
                        </span>
                        <small id="registerPassword-help" class="p-error">
                            {{ errors.password }}
                        </small>
                    </div>
                    <Button class="gablet-signin-button" type="submit" label="Submit" :disabled="registering" />
                    <small v-if="apiError" class="p-error">
                        {{ apiError }}
                    </small>
                </form>
            </template>
        </Card>
    </div>
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