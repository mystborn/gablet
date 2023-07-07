import { createApp } from 'vue';
import PrimeVue from 'primevue/config';
import App from './App.vue';
import Home from './pages/home/Home.vue';
import SignIn from './pages/sign-in/SignIn.vue';

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
import Inplace from 'primevue/inplace'
import InputText from 'primevue/inputtext'
import Editor from 'primevue/editor'
import Chips from 'primevue/chips'
import Menubar from 'primevue/menubar';
import Divider from 'primevue/divider';
import Password from 'primevue/password';

import Tooltip from 'primevue/tooltip'

import { router } from './router';

const app = createApp(App);

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

app.directive('tooltip', Tooltip)

app.use(PrimeVue);
app.use(router);

app.mount('#app')

export default {
    router
};