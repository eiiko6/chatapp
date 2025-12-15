<template>
  <div id="page">
    <router-view />
  </div>
</template>

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

@media screen and (max-width: 720px) {
  #page {
    padding: 0;
    padding-top: 20px;
    padding-bottom: 40px;
  }
}
</style>
