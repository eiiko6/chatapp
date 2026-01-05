<template>
    <div class="settings-page">
        <h1>{{ $t('settings-title') }}</h1>

        <UpdateAccountModal v-if="showUpdateModal" :user="user" @close="showUpdateModal = false"
            @updated="fetchUserData" />

        <h2>{{ $t('settings-account') }}</h2>
        <div v-if="user" class="info-card">
            <button class="update-btn" @click="showUpdateModal = true">{{ $t('settings-update-btn') }}</button>
            <p><strong>{{ $t('settings-label-username') }}</strong> {{ user.username }}</p>
            <p><strong>{{ $t('settings-label-email') }}</strong> {{ user.email }}</p>
        </div>
        <div v-else class="loading-state">
            <p>{{ $t('settings-loading') }}</p>
        </div>

        <h2>{{ $t('settings-language') }}</h2>
        <div class="input-group">
            <div class="lang-grid">
                <button v-for="lang in languages" :key="lang.code" class="lang-btn"
                    :class="{ active: currentLang === lang.code }" @click="changeLanguage(lang.code)">
                    {{ lang.name }}
                </button>
            </div>
        </div>

        <button class="logout-btn" @click="logout">
            <i class="fa-solid fa-right-from-bracket"></i>
            <span>{{ $t('settings-logout-btn') }}</span>
        </button>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { logout as authLogout } from '../store.ts'
import { getAuthData } from "../authStore.ts"
import type { User } from "../types"
import UpdateAccountModal from '../components/UpdateAccountModal.vue'
import { useFluent } from 'fluent-vue'
import { saveLocalePreference, getLocalePreference } from "../authStore.ts"
import { getSupportedLanguagesMetadata, setLanguage } from '../i18n'

const router = useRouter()
const user = ref<User | null>(null)
const showUpdateModal = ref(false)
const { $t } = useFluent()
const currentLang = ref('')

const languages = computed(() => getSupportedLanguagesMetadata())

async function fetchUserData() {
    try {
        const auth = await getAuthData()
        user.value = auth.user
    } catch (err) {
        console.error("Failed to load user data:", err)
    }
}

onMounted(async () => {
    const pref = await getLocalePreference()
    // Synchronize the UI state with the actual active language
    currentLang.value = pref || (navigator.language.split('-')[0])

    fetchUserData()
})

async function changeLanguage(code: string) {
    const actual = setLanguage(code)
    currentLang.value = actual
    await saveLocalePreference(actual)
}

function logout() {
    authLogout()
    router.push('/login')
}
</script>

<style scoped>
.settings-page {
    max-width: 720px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

h2 {
    margin-top: 20px;
}

.info-card {
    position: relative;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 1rem;
}

.update-btn {
    position: absolute;
    right: 10px;
    top: 10px;
}

.lang-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 0.5rem;
    margin-top: 0.5rem;
}

.lang-btn {
    background: var(--panel);
    border: 1px solid var(--border);
    color: var(--text);
    margin: 0;
}

.lang-btn.active {
    background: var(--accent);
    color: #000;
    border-color: var(--accent);
}

.logout-btn {
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 30px;
    padding: 10px 20px;
    color: var(--text);
    background-color: transparent;
    border: 1px solid var(--error);
    border-radius: var(--radius);
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

.loading-state {
    padding: 2rem;
    text-align: center;
    color: var(--muted);
}
</style>
