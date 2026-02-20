import { createRouter, createWebHistory } from "vue-router";
import Dashboard from "../pages/Dashboard.vue";
import Braindump from "../pages/Braindump.vue";
import Timer from "../pages/Timer.vue";

const routes = [
  {
    path: "/",
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

export default router;
