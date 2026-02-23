import { createRouter, createWebHistory } from "vue-router";
import Launcher from "../pages/Launcher.vue";
import Dashboard from "../pages/Dashboard.vue";
import Braindump from "../pages/Braindump.vue";
import Timer from "../pages/Timer.vue";

const routes = [
  {
    path: "/",
    name: "Launcher",
    component: Launcher,
  },
  {
    path: "/dashboard",
    name: "Dashboard",
    component: Dashboard,
  },
  {
    path: "/braindump",
    name: "Braindump",
    component: Braindump,
  },
  {
    path: "/timer",
    name: "Timer",
    component: Timer,
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  console.log("[Router] Navigation:", {
    from: from.path,
    to: to.path,
    name: to.name,
  });
  next();
});

router.afterEach((to) => {
  console.log("[Router] Navigation complete:", to.path);
});

export default router;
