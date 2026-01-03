<template>
  <div class="account-page">
    <h1>Your Account</h1>

    <div v-if="user" class="info-card">
      <p><strong>Username:</strong> {{ user.username }}</p>
      <p><strong>Email:</strong> {{ user.email }}</p>
    </div>
    <div v-else>
      <p>Loading account details...</p>
    </div>

    <button class="logout-btn" @click="logout">
      <i class="fa-solid fa-right-from-bracket"></i>
      <span>Logout</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { logout as authLogout } from '../store.ts'
import { getAuthData } from "../authStore.ts"
import type { User } from "../types"

const router = useRouter()
const user = ref<User | null>(null)

onMounted(async () => {
  try {
    const auth = await getAuthData()
    user.value = auth.user
  } catch (err) {
    console.error("Failed to load user data:", err)
  }
})

function logout() {
  authLogout()
  router.push('/login')
}
</script>

<style scoped>
.account-page {
  max-width: 720px;
  margin: 0 auto;
  padding: 2rem 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.info-card {
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 1rem;
}

.logout-btn {
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 20px;
  color: var(--text);
  background-color: transparent;
  border: 1px solid #ff5050;
  border-radius: 8px;
  width: fit-content;
  transition: all 0.2s ease;
}

.logout-btn:hover {
  color: rgba(255, 80, 80, 0.8);
}

.logout-btn:hover i {
  color: rgba(255, 80, 80, 0.8);
}

.logout-btn i {
  font-size: 1.25rem;
}
</style>
