<script setup lang="ts">
import { inject } from 'vue';
import SignInForm from './SignInForm.vue';
import type { LoginResponse } from '@/api/auth';

const dialogRef = inject('dialogRef') as { value: { close: (data?: any) => void } };

const onLogin = (response: LoginResponse) => {
    dialogRef.value.close(response);
}
</script>

<template>
    <div>
        <SignInForm @login="onLogin" />
    </div>
</template>