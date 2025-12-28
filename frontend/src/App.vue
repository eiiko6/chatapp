<template>
  <div id="page">
    <main id="content">
      <router-view />
    </main>

    <nav id="bottom-nav">
      <router-link to="/" class="nav-item" aria-label="Home">
        <i class="fa-solid fa-message"></i>
      </router-link>

      <router-link to="/friendlist" class="nav-item" aria-label="Friends">
        <i class="fa-solid fa-user-group"></i>
      </router-link>

      <button class="nav-item logout" @click="logout" aria-label="Logout">
        <i class="fa-solid fa-right-from-bracket"></i>
      </button>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { logout as authLogout } from './stores/auth.ts'

const router = useRouter()

function logout() {
  authLogout()
  router.push('/login')
}
</script>

<style scoped>
#page {
  min-height: 100vh;
  background: var(--bg);
  display: flex;
  justify-content: center;
}

#content {
  width: 100%;
  max-width: 1100px;
  padding: 2rem;
  padding-bottom: 120px;
}

#bottom-nav {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);

  display: flex;
  gap: 28px;

  padding: 14px 22px;
  background: color-mix(in srgb, var(--panel) 85%, transparent);
  border: 1px solid var(--border);
  border-radius: 999px;

  backdrop-filter: blur(10px);
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

  color: var(--muted);
  font-size: 1.25rem;
  border-radius: 50%;

  transition:
    color 0.2s ease,
    background-color 0.2s ease,
    transform 0.15s ease;
}

.nav-item:hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.04);
  transform: translateY(-2px);
}

.nav-item:not(.router-link-active):hover {
  color: var(--text);
  background: rgba(255, 255, 255, 0.04);
  transform: translateY(-2px);
}

.router-link-active {
  color: var(--accent);
}

.logout:hover {
  color: rgba(255, 80, 80, 0.8);
}

@media (max-width: 720px) {
  #content {
    padding: 1.2rem;
    padding-bottom: 130px;
  }

  #bottom-nav {
    bottom: 16px;
  }
}
</style>
