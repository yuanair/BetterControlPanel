import {createRouter, createWebHistory} from 'vue-router'
import WelcomeView from '../views/Welcome.vue'
import LevelSelectView from '../views/LevelSelect.vue'

const routes = [
    {
        path: '/',
        name: 'welcome',
        component: WelcomeView
    },
    {
        path: '/levels',
        name: 'levels',
        component: () => LevelSelectView
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router