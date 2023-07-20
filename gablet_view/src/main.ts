import { createApp } from 'vue';
import PrimeVue from 'primevue/config';
import App from './App.vue';
import i18next from 'i18next';
import LanguageDetector from 'i18next-browser-languagedetector'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import 'primevue/resources/primevue.min.css';
import 'primevue/resources/themes/lara-light-purple/theme.css';
import 'primeicons/primeicons.css';
import 'primeflex/primeflex.min.css'
import './assets/main.css';

import Panel from 'primevue/panel'
import Toolbar from 'primevue/toolbar'
import Button from 'primevue/button'
import Dropdown from 'primevue/dropdown'
import AutoComplete from 'primevue/autocomplete'
import Card from 'primevue/card'
import Tag from 'primevue/tag'
import Dialog from 'primevue/dialog'
import DialogService from 'primevue/dialogservice';
import DynamicDialog from 'primevue/dynamicdialog';
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Editor from 'primevue/editor'
import Chips from 'primevue/chips'
import Menubar from 'primevue/menubar';
import Divider from 'primevue/divider';
import Password from 'primevue/password';
import Message from 'primevue/message';
import ProgressSpinner from 'primevue/progressspinner';

import Tooltip from 'primevue/tooltip'

import { router } from './router';
import useI18n from './i18n';

const app = createApp(App);
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

app.component('Panel', Panel)
app.component('Toolbar', Toolbar)
app.component('Button', Button)
app.component('Dropdown', Dropdown)
app.component('AutoComplete', AutoComplete)
app.component('Card', Card)
app.component('Tag', Tag)
app.component('Dialog', Dialog)
app.component('Inplace', Inplace)
app.component('InputText', InputText)
app.component('Password', Password)
app.component('Editor', Editor)
app.component('Chips', Chips);
app.component('Menubar', Menubar);
app.component('Divider', Divider);
app.component('Message', Message);
app.component('ProgressSpinner', ProgressSpinner);

app.directive('tooltip', Tooltip);

app.use(pinia);
app.use(PrimeVue);
app.use(DialogService);
app.use(router);

useI18n(app);

app.mount('#app');

export default {
    router
};