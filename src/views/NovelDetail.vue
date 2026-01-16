<template>
  <div class="container">
    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
      <el-page-header @back="goBack" :content="$t('novel.detail')" />
      <LanguageSwitcher />
    </div>
    <el-tabs v-model="activeTab">
      <el-tab-pane :label="$t('tabs.players')" name="players">
        <PlayerManager :novel-id="novelId" />
      </el-tab-pane>
      <el-tab-pane :label="$t('tabs.monsters')" name="monsters">
        <MonsterManager :novel-id="novelId" />
      </el-tab-pane>
      <el-tab-pane :label="$t('category.management')" name="categories">
        <CategoryManager :novel-id="novelId" />
      </el-tab-pane>
      <el-tab-pane :label="$t('tabs.items')" name="items">
        <ItemManager :novel-id="novelId" />
      </el-tab-pane>
       <el-tab-pane :label="$t('tabs.maps')" name="maps">
        <MapManager :novel-id="novelId" />
      </el-tab-pane>
      <el-tab-pane :label="$t('tabs.buffs')" name="buffs">
        <BuffManager :novel-id="novelId" />
      </el-tab-pane>
      <el-tab-pane :label="$t('tabs.calculator')" name="calculator">
        <DamageCalculator :novel-id="novelId" :is-active="activeTab === 'calculator'" />
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import PlayerManager from '../components/PlayerManager.vue'
import MonsterManager from '../components/MonsterManager.vue'
import ItemManager from '../components/ItemManager.vue'
import MapManager from '../components/MapManager.vue'
import CategoryManager from '../components/CategoryManager.vue'
import BuffManager from '../components/BuffsManager.vue'
import DamageCalculator from '../components/DamageCalculator.vue'
import LanguageSwitcher from '../components/LanguageSwitcher.vue'

const router = useRouter()
const route = useRoute()
const activeTab = ref('players')

const novelId = computed(() => Number(route.params.id))

const goBack = () => {
  router.push('/')
}
</script>

<style scoped>
.container {
  padding: 20px;
}
</style>
