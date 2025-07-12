import { createMemoryHistory, createRouter } from "vue-router";
import TimerView from "./pages/TimerView.vue";
import ErrorView from "./pages/ErrorView.vue";

const routes = [
  { path: "/", component: TimerView },
  { path: "/error", component: ErrorView },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
