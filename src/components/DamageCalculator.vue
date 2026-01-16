<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="12">
        <h3>{{ $t('calculator.attacker') }}</h3>
        <el-select v-model="selectedPlayerId" :placeholder="$t('calculator.select_attacker')" @change="onPlayerChange">
          <el-option v-for="p in players" :key="p.id" :label="p.name" :value="p.id" />
        </el-select>
        <div v-if="selectedPlayer" class="stats-box">
          <p>{{ $t('player.attack') }}: {{ selectedPlayer.attack }}</p>
          <p>{{ $t('player.strength') }}: {{ selectedPlayer.strength }}</p>
          <p>{{ $t('player.agility') }}: {{ selectedPlayer.agility }}</p>
          <p>{{ $t('player.intelligence') }}: {{ selectedPlayer.intelligence }}</p>
        </div>
        <div style="margin-top: 20px">
          <el-checkbox v-model="isCrit">{{ $t('calculator.crit_hit') }}</el-checkbox>
          <el-checkbox v-model="isFatal">{{ $t('calculator.fatal_blow') }}</el-checkbox>
        </div>
      </el-col>
      <el-col :span="12">
        <h3>{{ $t('calculator.defender') }}</h3>
        <el-select v-model="selectedMonsterId" :placeholder="$t('calculator.select_defender')" @change="onMonsterChange">
          <el-option v-for="m in monsters" :key="m.id" :label="m.name" :value="m.id" />
        </el-select>
        <div v-if="selectedMonster" class="stats-box">
          <p>{{ $t('player.hp') }}: {{ selectedMonster.hp }}</p>
          <p>{{ $t('monster.defense') }}: {{ selectedMonster.defense }}</p>
          <p>{{ $t('monster.damage_reduction') }}: {{ selectedMonster.damage_reduction }}</p>
        </div>
      </el-col>
    </el-row>

    <div class="result-box" v-if="damageResult !== null">
      <h2>{{ $t('calculator.result') }}: {{ damageResult }}</h2>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps({
  novelId: Number,
  isActive: Boolean
})

const players = ref([])
const monsters = ref([])
const selectedPlayerId = ref(null)
const selectedMonsterId = ref(null)
const isCrit = ref(false)
const isFatal = ref(false)

const selectedPlayer = computed(() => players.value.find(p => p.id === selectedPlayerId.value))
const selectedMonster = computed(() => monsters.value.find(m => m.id === selectedMonsterId.value))

const damageResult = computed(() => {
  if (!selectedPlayer.value || !selectedMonster.value) return null
  
  // Simple damage formula: (Atk - Def) * Multipliers
  let damage = selectedPlayer.value.attack - selectedMonster.value.defense
  if (damage < 0) damage = 0 

  // Multipliers
  let multiplier = 1.0
  if (isCrit.value) multiplier *= 1.5
  if (isFatal.value) multiplier *= 2.0

  damage = Math.floor(damage * multiplier) - selectedMonster.value.damage_reduction
  
  // Minimum damage is 1
  if (damage < 1) damage = 1
  
  return damage
})

const loadData = async () => {
  if (!props.novelId) return
  try {
    players.value = await invoke('get_players', { novelId: props.novelId })
    monsters.value = await invoke('get_monsters', { novelId: props.novelId })
  } catch (e) {
    console.error(e)
  }
}

const onPlayerChange = () => {}
const onMonsterChange = () => {}

watch(() => props.novelId, loadData)
watch(() => props.isActive, (newVal) => {
  if (newVal) {
    loadData()
  }
})

onMounted(loadData)
</script>

<style scoped>
.stats-box {
  margin-top: 10px;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
}
.result-box {
  margin-top: 30px;
  text-align: center;
  padding: 20px;
  background-color: #f0f9eb;
  border: 1px solid #67c23a;
  border-radius: 4px;
  color: #67c23a;
}
</style>
