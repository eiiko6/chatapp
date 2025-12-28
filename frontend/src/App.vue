<template>
  <div id="page">
    <main id="content">
      <router-view />
    </main>

    <footer v-if="!$route.meta.hideNavbar">
      <Navbar />
    </footer>
  </div>
</template>

<script setup lang="ts">
import Navbar from './components/Navbar.vue';
</script>

<style scoped>
#page {
  background: var(--bg);
  height: 100dvh;
  display: flex;
  flex-direction: column;
  align-items: center;
}

#content {
  width: 100%;
  max-width: 1100px;
  padding: 2rem;

  flex: 1;
  overflow-y: auto;
}

footer {
  width: 100%;
  display: flex;
  justify-content: center;
  padding-bottom: 24px;
  background: var(--bg);
}

@media (max-width: 720px) {
  #content {
    padding: 1.2rem;
  }

  footer {
    padding-bottom: 16px;
  }
}
</style>
