<script setup lang="ts">
import useAuthStore from '@/stores/useAuthStore';
import { isLoggedIn } from '@/utils/auth';
import { type ButtonEmits, type ButtonProps } from 'primevue/button';
import { useDialog } from 'primevue/usedialog';
import { useRouter } from 'vue-router';
import SignInDialog from '../SignInDialog.vue';
import { useTranslation } from 'i18next-vue';
import { ref } from 'vue';
import { devLog } from '@/utils/errors';

interface ProfileIconProps extends /* @vue-ignore */ ButtonProps {};

const props = defineProps<ProfileIconProps>();
const dialog = useDialog();
const router = useRouter();
const auth = useAuthStore();
const { t } = useTranslation();
const canClick = ref(true);

const onClick = async (ev: MouseEvent) => {
    try {
        canClick.value = false;

        if (await isLoggedIn(auth, t)) {
            router.push('/profile');
            canClick.value = true;
            return;
        }

        dialog.open(
            SignInDialog,
            {
                props: {
                    header: t('signin.signIn'),
                    modal: true
                },
                onClose: async (options) => {
                    canClick.value = true;
                    if (options?.data && await isLoggedIn(auth, t)) {
                        router.push('/profile');
                    }
                },
                onHide: () =>  {
                    canClick.value = true;
                }
            }
        );
    } catch(err) {
        devLog(err);
    }
}

</script>

<template>
    <Button
        v-bind="props"
        :disabled="!canClick" 
        @click="onClick" 
        icon="pi pi-user"
        aria-label="Profile"
        text />
</template>