<script setup lang="ts">
import { useAuthStore } from "./stores/auth";

const auth = useAuthStore();
</script>

<template>
  <!-- You can later replace this with a real layout -->
  <router-view />
</template>
