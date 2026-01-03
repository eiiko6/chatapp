<template>
    <div class="login-page">
        <form class="login-card" @submit.prevent="submit">
            <h1>{{ $t('auth-login-title') }}</h1>
            <div class="input-group">
                <label>{{ $t('auth-email') }}</label>
                <input v-model="email" :placeholder="$t('auth-email').toLowerCase()" />
            </div>
            <div class="input-group">
                <label>{{ $t('auth-password') }}</label>
                <input v-model="password" type="password" :placeholder="$t('auth-password').toLowerCase()" />
            </div>
            <button type="submit">{{ $t('auth-login-btn') }}</button>
            <p v-if="errorMessage" class="error-message">{{ errorMessage }}</p>
            <p class="register-link">
                {{ $t('auth-no-account') }} <router-link to="/register">{{ $t('auth-register-title') }}</router-link>
            </p>
        </form>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { login } from '../store.ts'
import { useRouter } from "vue-router";
import { useFluent } from 'fluent-vue';

const email = ref("");
const password = ref("");
const errorMessage = ref("");

const router = useRouter();

const { $t } = useFluent();

async function submit() {
    errorMessage.value = "";
    try {
        await login(email.value, "", password.value);
        router.push("/");
    } catch (err: any) {
        errorMessage.value = err?.message || $t('auth-error-unknown');
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

.register-link {
    font-size: 0.9rem;
    text-align: center;
    color: var(--text-muted);
}

.register-link a {
    color: var(--accent);
    margin-left: 5px;
}

.error-message {
    color: var(--error);
    font-size: 0.9rem;
    text-align: center;
    margin-top: 0.5rem;
}
</style>
