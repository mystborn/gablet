<script setup lang="ts">
import { useRouter, useRoute } from 'vue-router'
import api from '@/api/api'
import { ref } from 'vue';
import { useTranslation } from 'i18next-vue';
import Header from '@/components/Header.vue';
import Footer from '@/components/Footer.vue';

const router = useRouter();
const route = useRoute();

const { t } = useTranslation();

const validating = ref(true);
const success = ref(false);
const message = ref(t('validate.validating'));

const validate = async () => {
    const token = route.query.token?.toString();
    const username = route.query.username?.toString();

    if (!token || !username) {
        validating.value = false;
        message.value = t('validate.invalidUrl');
        success.value = false;

        return;
    }

    try {
        let response = await api.auth.validate_account({ username, token });
        if (response.error) {
            success.value = false;
            message.value = t('validate.error', { error: response.error.error_message });
        } else {
            success.value = true;
            message.value = t('validate.success', { account: username });
        }
    } catch (err) {
        success.value = false;
        message.value = t('validate.error', { error: new String(err) });
    } finally {
        validating.value = false;
    }
}

validate();
</script>

<template>
    <div id="gablet-validate" class="flex flex-grow-1 justify-content-center align-items-center">
        <div>
            <Card>
                <template #title>{{ t('validate.validateAccount') }}</template>
                <template #content>
                    <Divider />
                    <div v-if="validating" class="flex flex-column align-items-center">
                        <ProgressSpinner />
                        <p class="text-2xl mt-3">{{ message }}</p>
                    </div>
                    <div v-else class="flex flex-column align-items-center">
                        <i v-if="success" class="pi pi-check gablet-validate-icon success"></i>
                        <i v-else class="pi pi-times-circle gablet-validate-icon error"></i>
                        <p class="text-2xl mt-3">{{ message }}</p>
                    </div>
                </template>
            </Card>
        </div>
    </div>
</template>

<style>
#gablet-validate {
    margin-left: 10%;
    margin-right: 10%;
}

#gablet-validate .p-message-icon {
    display: none;
}

#gablet-validate .p-divider {
    margin-top: 0px;
}

.gablet-validate-icon {
    font-size: 9rem;
}

.gablet-validate-icon.success {
    color: var(--green-500);
}

.gablet-validate-icon.error {
    color: var(--red-500);
}

@keyframes p-progress-spinner-color {
    0% {
        stroke: var(--primary-color);
    }

    100% {
        stroke: var(--primary-color);
    }
}
</style>