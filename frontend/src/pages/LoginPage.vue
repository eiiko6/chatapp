<script setup lang="ts">
import { ref } from "vue";
import { useAuthStore } from "../stores/auth";
import { useRouter } from "vue-router";

const email = ref("");
const username = ref("");
const password = ref("");

const auth = useAuthStore();
const router = useRouter();

async function submit() {
  await auth.login(email.value, username.value, password.value);
  router.push("/");
}
</script>

<template>
  <div class="login-page">
    <form class="login-card" @submit.prevent="submit">
      <h1>Login</h1>

      <input v-model="email" placeholder="email" />
      <input v-model="password" type="password" placeholder="password" />

      <button type="submit">Login</button>
    </form>
  </div>
</template>

<style scoped>
.login-page {
  height: 100%;
  display: grid;
  place-items: center;
}

.login-card {
  width: 100%;
  max-width: 360px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 2rem;

  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.login-card h1 {
  font-size: 1.4rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  text-align: center;
}
</style>
