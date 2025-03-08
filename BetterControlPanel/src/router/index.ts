import {createRouter, createWebHistory} from 'vue-router'
import WelcomeView from '../views/Welcome.vue'
import LevelSelectView from '../views/LevelSelect.vue'

const routes = [
    {
        path: '/',
        name: 'welcome',
        displayName: 'Welcome',
        component: WelcomeView
    },
    {
        path: '/levels',
        name: 'levels',
        displayName: 'Levels',
        component: () => LevelSelectView
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router