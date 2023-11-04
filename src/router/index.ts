import { createRouter,createWebHashHistory } from 'vue-router'

const routes = [
    {
        path:'/',
        redirect:'/index'
    },
    {
        path:'/index',
        component:()=>import("./../views/index.vue")
    },
    {
        path:'/sub',
        children:[
            {
                path:'editor',
                component:()=>import('./../views/page/editor.vue')
            },
            {
                path:'lnk/:row/:col',
                component:()=>import('./../views/page/lnk.vue')
            },
            {
                path:'search',
                component:()=>import('./../views/page/search.vue')
            },
            {
                path:'weather',
                component:()=>import('./../views/page/weather.vue')
            },
            {
                path:'roulette',
                component:()=>import('./../views/page/roulette.vue')
            },
            {
                path:'netspeed',
                component:()=>import('./../views/page/netspeed.vue')
            },
            {
                path:'sysinfo',
                component:()=>import('./../views/page/sysinfo.vue')
            },
            {
                path:'wallpaper',
                component:()=>import('./../views/page/wallpaper.vue')
            },
            {
                path:'program',
                component:()=>import('./../views/page/program.vue')
            },
        ]
    }
]

const router = createRouter({
    routes,
    history:createWebHashHistory()
})

export default router