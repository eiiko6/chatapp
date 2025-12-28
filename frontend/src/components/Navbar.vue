<template>
  <div>
    <nav id="bottom-nav">
      <router-link to="/rooms/none" class="nav-item" :class="{ 'router-link-active': $route.name === 'chat' }">
        <i class="fa-solid fa-message"></i>
      </router-link>

      <router-link to="/friendlist" class="nav-item">
        <i class="fa-solid fa-user-group"></i>
      </router-link>

      <button class="nav-item logout" @click="logout">
        <i class="fa-solid fa-right-from-bracket"></i>
      </button>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { logout as authLogout } from '../stores/auth.ts'

const router = useRouter()

function logout() {
  authLogout()
  router.push('/login')
}
</script>

<style scoped>
#bottom-nav {
  display: flex;
  gap: 28px;

  padding: 5px 22px;
  background: var(--panel);
  border: 1px solid var(--border);
  border-radius: 100vh;
  z-index: 50;
}

.nav-item {
  all: unset;
  cursor: pointer;

  width: 44px;
  height: 44px;

  display: flex;
  align-items: center;
  justify-content: center;

  font-size: 1.25rem;
  border-radius: 50%;

  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    transform 0.15s ease;
}

.nav-item i {
  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    transform 0.15s ease;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.04);
}

.nav-item:not(.router-link-active):hover {
  background: rgba(255, 255, 255, 0.04);
}

.nav-item:not(.router-link-active):hover i {
  color: var(--text);
}

.router-link-active i {
  color: var(--accent);
}

.nav-item.logout:hover i {
  color: rgba(255, 80, 80, 0.8);
}

@media (max-width: 720px) {
  #bottom-nav {
    bottom: 16px;
  }
}
</style>
