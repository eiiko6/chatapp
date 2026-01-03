<template>
  <div class="login-page">
    <form class="login-card" @submit.prevent="submit" novalidate>
      <h1>{{ $t('auth-register-title') }}</h1>

      <div class="input-group">
        <label>{{ $t('auth-email') }}</label>
        <input v-model="email" type="email" :placeholder="$t('auth-email').toLowerCase()" required />
      </div>

      <div class="input-group">
        <label>{{ $t('auth-username') }}</label>
        <input v-model="username" :placeholder="$t('auth-username').toLowerCase()" required />
      </div>

      <div class="input-group">
        <label>{{ $t('auth-password') }}</label>
        <input v-model="password" type="password" :placeholder="$t('auth-password').toLowerCase()" required />
        <input v-model="confirmPassword" type="password" :placeholder="$t('auth-confirm-password')" required />
      </div>

      <button type="submit">{{ $t('auth-register-btn') }}</button>

      <p class="login-link">
        {{ $t('auth-has-settings') }} <router-link to="/login">{{ $t('auth-login-title') }}</router-link>
      </p>

      <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { register } from '../store.ts'
import { useRouter } from "vue-router";
import { useFluent } from 'fluent-vue';

const { $t } = useFluent();

const email = ref("");
const username = ref("");
const password = ref("");
const confirmPassword = ref("");
const errorMessage = ref("");

const router = useRouter();

async function submit(event: Event) {
  errorMessage.value = "";

  const form = event.target as HTMLFormElement;

  // Check password length and email
  if (!form.checkValidity()) {
    if (password.value.length < 8) {
      errorMessage.value = $t('auth-error-password-length');
    } else {
      errorMessage.value = $t('auth-error-email-invalid');
    }
    return;
  }

  // Check password match
  if (password.value !== confirmPassword.value) {
    errorMessage.value = $t('auth-error-password-match');
    return;
  }

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
  color: var(--error);
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
