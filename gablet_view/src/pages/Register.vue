<script setup lang="ts">
import { ref } from 'vue';
import { useForm } from 'vee-validate';
import * as yup from 'yup';

const schema = yup.object({
    username: yup.string().required().label("Username"),
    email: yup.string().email().required().label("Email"),
    password: yup.string().required().min(8)
});

const { defineComponentBinds, handleSubmit, resetForm, errors, validate } = useForm({ validationSchema: schema });

const username = defineComponentBinds('registerUsername');
const email = defineComponentBinds('registerEmail');
const password = defineComponentBinds('registerPassword');

const onSubmit = handleSubmit((values) => {
    console.log("submitted with", values);
});

const onSubmit2 = async (vals: any) => {
    let result = await validate();
    console.log("validated:", result);
    console.log("Submitting", vals);
}

</script>

<template>
    <div class="card flex justify-content-center">
        <form @submit.prevent="onSubmit2">
            <div>
                <span class="p-float-label">
                    <InputText 
                        type="text" 
                        v-bind="username" 
                        id="registerUsername"
                        :class="{ 'p-invalid': errors.registerUsername }" />
                    <label for="registerUsername">Username</label>
                </span>
                <small id="registerUsername-help" class="p-error">
                    {{ errors.registerUsername }}
                </small>
            </div>
            <div>
                <span class="p-float-label">
                    <InputText 
                        type="text" 
                        v-bind="email" 
                        id="registerEmail"
                        :class="{ 'p-invalid': errors.registerEmail }" />
                    <label for="registerEmail">Email</label>
                </span>
                <small id="registerEmail-help" class="p-error">
                    {{ errors.registerEmail }}
                </small>
            </div>
            <div>
                <span class="p-float-label">
                    <Password 
                        v-bind="password" 
                        toggleMask
                        inputId="registerPassword"
                        :class="{ 'p-invalid': errors.registerPassword }" />
                    <label for="registerPassword">Password</label>
                </span>
                <small id="registerPassword-help" class="p-error">
                    {{ errors.registerPassword }}
                </small>
            </div>
            <Button type="submit" label="Submit" />
        </form>
    </div>
</template>