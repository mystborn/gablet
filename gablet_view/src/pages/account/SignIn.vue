<script setup lang="ts">
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import * as yup from 'yup';
import api from '@/api/api';
import { useRoute, useRouter } from 'vue-router';

const schema = yup.object({
    username: yup.string().required().label("Username or Email"),
    password: yup.string().required().min(8).label("Password")
});

const router = useRouter();
const route = useRoute();

const formState = useForm({ validationSchema: schema });

const { defineComponentBinds, handleSubmit, errors } = formState;

const username = defineComponentBinds('username');
const password = defineComponentBinds('password');

const onSubmit = handleSubmit(async (values) => {
    console.log("submitted with", values);
    try {
        let result = await api.auth.login({ username: values.username, password: values.password });
        if (result.error) {
            console.log("Failed to register account", result.error);
            return;
        }

        const redirect = route.query.link?.toString();
        if(redirect) {
            window.location.href = decodeURI(redirect);
            return;
        }
    } catch(err) {
        console.log("Failed to register account", err);
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
                    <Button class="gablet-signin-button" type="submit" label="Submit" />
                </form>
            </template>
        </Card>
    </div>
</template>

<style scoped>

.gablet-signin-container {
    padding: 1.5rem;
}

.gablet-signin-input {
    margin-bottom: 1.5rem;
}

.gablet-signin-input-text {
    width: 100%;
}

.gablet-signin-button {
    width: 100%;
}

.centered {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

</style>