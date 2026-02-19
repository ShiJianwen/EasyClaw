import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      path: "/home",
      name: "home",
      component: () => import("../components/HomePage.vue"),
    },
    {
      path: "/chat",
      name: "chat",
      component: () => import("../components/ChatView.vue"),
    },
  ],
});

export default router;
