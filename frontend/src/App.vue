<template>
  <div id="page">
    <router-view />

    <div id="absolute">
      <button class="logout" @click="logout()">Logout</button>

      <router-link to="." class="button">
        Home
      </router-link>

      <router-link to="/friendlist" class="button">
        Friendlist
      </router-link>
    </div>
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
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  padding: 2rem;
  box-sizing: border-box;
  background: var(--background);
}

#absolute {
  position: absolute;
  left: 0;
  top: 0;
  margin: 30px;
}

@media screen and (max-width: 720px) {
  #page {
    padding: 0;
    padding-top: 20px;
    padding-bottom: 40px;
  }
}
</style>
