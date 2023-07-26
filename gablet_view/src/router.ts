import { createRouter, createWebHistory } from "vue-router";
import Home from './pages/home/Home.vue';
import SignIn from './pages/account/SignIn.vue';
import Register from './pages/Register.vue';
import Validate from './pages/account/Validate.vue';
import Profile from './pages/account/Profile.vue';

const routes = [
    { path: '/', name: 'Home', component: Home },
    { path: '/sign-in', name: 'SignIn', component: SignIn },
    { path: '/register', name: 'Register', component: Register },
    { path: '/validate', name: 'Validate', component: Validate },
    { path: '/profile', component: Profile }
];

export const router = createRouter({
    history: createWebHistory(),
    routes
});