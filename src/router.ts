import { createMemoryHistory, createRouter } from "vue-router";
import TimerView from "./routes/TimerView.vue";

const routes = [{ path: "/", component: TimerView }];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

export default router;
