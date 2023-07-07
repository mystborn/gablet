import { createRouter, createWebHistory } from "vue-router";
import Home from './pages/home/Home.vue';
import SignIn from './pages/sign-in/SignIn.vue';
import Register from './pages/Register.vue';

const routes = [
    { path: '/', name: 'Home', component: Home },
    { path: '/sign-in', name: 'SignIn', component: SignIn },
    { path: '/register', name: 'Register', component: Register }
];

export const router = createRouter({
    history: createWebHistory(),
    routes
});