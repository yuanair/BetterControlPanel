import {createRouter, createWebHistory} from 'vue-router'
import WelcomeView from '../views/Welcome.vue'
import LevelSelectView from '../views/LevelSelect.vue'
import MathView from '../views/levels/Math.vue'
import PhysicsView from '../views/levels/Physics.vue'
import ChemistryView from '../views/levels/Chemistry.vue'
import NotFoundView from '../views/NotFound.vue'

const routes = [
    {path: '/', name: 'welcome', component: WelcomeView},
    {path: '/levels', name: 'levels', component: LevelSelectView},
    {path: '/levels/math', name: 'levels-math', component: MathView},
    {path: '/levels/physics', name: 'levels-physics', component: PhysicsView},
    {path: '/levels/chemistry', name: 'levels-chemistry', component: ChemistryView},
    {path: '/levels/:id', name: 'not-found', component: NotFoundView},
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router