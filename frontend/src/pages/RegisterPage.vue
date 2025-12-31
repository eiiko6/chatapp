<template>
  <div class="login-page">
    <form class="login-card" @submit.prevent="submit">
      <h1>Register</h1>

      <input v-model="email" type="email" placeholder="email" required />
      <input v-model="username" placeholder="username" required />
      <input v-model="password" type="password" placeholder="password" required />

      <button type="submit">Create Account</button>

      <p class="login-link">
        Already have an account? <router-link to="/login">Login</router-link>
      </p>

      <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { register } from '../store.ts'
import { useRouter } from "vue-router";

const email = ref("");
const username = ref("");
const password = ref("");
const errorMessage = ref("");

const router = useRouter();

async function submit() {
  errorMessage.value = "";
  try {
    await register(email.value, username.value, password.value);

    router.push("/");
  } catch (err: any) {
    errorMessage.value = err?.message || "An unknown error occurred";
  }
}
</script>

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

.error-message {
  color: red;
  font-size: 0.9rem;
  text-align: center;
  margin-top: 0.5rem;
}


.login-link {
  font-size: 0.9rem;
  text-align: center;
  color: var(--text-muted);
}

.login-link a {
  color: var(--accent);
  margin-left: 5px;
}
</style>
