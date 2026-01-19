<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="12">
        <h3>{{ $t('common.player') }} (Left)</h3>
        <el-select v-model="selectedPlayerId" :placeholder="$t('calculator.select_attacker')" @change="onPlayerChange">
          <el-option v-for="p in players" :key="p.id" :label="p.name" :value="p.id" />
        </el-select>
        
        <div v-if="selectedPlayer" class="stats-box">
          <div style="margin-bottom: 15px">
             <p>{{ $t('player.hp') }}: {{ selectedPlayer.hp }} / {{ selectedPlayer.max_hp }} ({{ Math.floor(selectedPlayer.hp / selectedPlayer.max_hp * 100) }}%)</p>
             <p>{{ $t('player.shield') }}: {{ selectedPlayer.shield }} / {{ selectedPlayer.max_shield }}</p>
          </div>
          
          <el-divider>Buffs</el-divider>
          <el-checkbox-group v-model="activeBuffIds">
            <el-checkbox v-for="buff in buffs" :key="buff.id" :label="buff.id">{{ buff.name }}</el-checkbox>
          </el-checkbox-group>
          
          <el-divider>Final Stats</el-divider>
          <p>{{ $t('player.hp') }}: {{ finalPlayerStats.hp }} / {{ finalPlayerStats.max_hp }}</p>
          <p>{{ $t('player.shield') }}: {{ finalPlayerStats.shield }} / {{ finalPlayerStats.max_shield }}</p>
          <p>{{ $t('player.attack') }}: {{ finalPlayerStats.attack }} <span v-if="finalPlayerStats.attack > selectedPlayer.attack" style="color: green">(+{{ finalPlayerStats.attack - selectedPlayer.attack }})</span></p>
          <p>{{ $t('player.phys_defense') }}: {{ finalPlayerStats.phys_defense }}</p>
          <p>{{ $t('player.mag_defense') }}: {{ finalPlayerStats.mag_defense }}</p>
          <p>{{ $t('player.strength') }}: {{ finalPlayerStats.strength }}</p>
        </div>
        
        <div style="margin-top: 20px">
          <el-checkbox v-model="isCrit">{{ $t('calculator.crit_hit') }}</el-checkbox>
          <el-checkbox v-model="isFatal">{{ $t('calculator.fatal_blow') }}</el-checkbox>
        </div>
      </el-col>
      <el-col :span="12">
        <h3>{{ $t('common.monster') }} (Right)</h3>
        <el-select v-model="selectedMonsterId" :placeholder="$t('calculator.select_defender')" @change="onMonsterChange">
          <el-option v-for="m in monsters" :key="m.id" :label="m.name" :value="m.id" />
        </el-select>
        <div v-if="selectedMonster" class="stats-box">
          <p>{{ $t('player.hp') }}: {{ selectedMonster.hp }}</p>
          <p>{{ $t('player.attack') }}: {{ selectedMonster.attack }}</p>
          <p>{{ $t('monster.defense') }}: {{ selectedMonster.defense }}</p>
          <p>{{ $t('monster.damage_reduction') }}: {{ selectedMonster.damage_reduction }}%</p>
          <p>Fixed Reduction: {{ selectedMonster.fixed_damage_reduction || 0 }}</p>
        </div>
      </el-col>
    </el-row>

    <el-row :gutter="20" style="margin-top: 30px">
       <el-col :span="12">
          <el-card class="result-card" shadow="hover">
            <template #header>
               <div class="card-header">
                 <span>Player -> Monster Damage</span>
               </div>
            </template>
            <h2 class="damage-text">{{ playerToMonsterDamage !== null ? playerToMonsterDamage : '-' }}</h2>
          </el-card>
       </el-col>
       <el-col :span="12">
          <el-card class="result-card" shadow="hover">
             <template #header>
               <div class="card-header">
                 <span>Monster -> Player Damage</span>
               </div>
            </template>
             <h2 class="damage-text">{{ monsterToPlayerDamage !== null ? monsterToPlayerDamage : '-' }}</h2>
          </el-card>
       </el-col>
    </el-row>
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
const buffs = ref([])
const activeBuffIds = ref([])
const currentHpPct = ref(100)
const selectedPlayerId = ref(null)
const selectedMonsterId = ref(null)
const isCrit = ref(false)
const isFatal = ref(false)

const selectedPlayer = computed(() => players.value.find(p => p.id === selectedPlayerId.value))
const selectedMonster = computed(() => monsters.value.find(m => m.id === selectedMonsterId.value))

const evaluateFormula = (formula, context) => {
  try {
    // Replace variables in formula with values from context
    // Simple replacement for now, better to use a parser but JS eval is fine for local tool
    // Variables: hp, max_hp, hp_pct, level, str, agi, int, vit, spi, attack, defense
    
    // Create a function with the context keys as arguments
    const keys = Object.keys(context)
    const values = Object.values(context)
    
    // Using Function constructor is safer than eval() because it creates a new scope
    // But we need to make sure formula uses matching variable names
    // Let's assume formula uses "hp_pct" directly.
    
    const func = new Function(...keys, `return ${formula}`)
    return func(...values)
  } catch (e) {
    console.warn('Formula evaluation failed', formula, e)
    return 0
  }
}

const finalPlayerStats = computed(() => {
  if (!selectedPlayer.value) return {}
  
  // Clone stats
  const stats = { ...selectedPlayer.value }
  
  // Base context
  const context = {
    hp: stats.hp, // Current HP
    max_hp: stats.max_hp,
    hp_pct: stats.max_hp > 0 ? Math.floor((stats.hp / stats.max_hp) * 100) : 0,
    shield: stats.shield, // Current Shield
    max_shield: stats.max_shield,
    level: stats.level,
    str: stats.strength,
    agi: stats.agility,
    int: stats.intelligence,
    vit: stats.vitality,
    spi: stats.spirit,
    attack: stats.attack,
    defense: stats.phys_defense, // Alias
    phys_defense: stats.phys_defense,
    mag_defense: stats.mag_defense
  }
  
  // Apply buffs
  activeBuffIds.value.forEach(buffId => {
    const buff = buffs.value.find(b => b.id === buffId)
    if (!buff || !buff.effects) return
    
    // Parse effects if string (db returns JSONB as object usually, but check)
    let effects = buff.effects
    if (typeof effects === 'string') {
        try { effects = JSON.parse(effects) } catch(e) { return }
    }
    
    effects.forEach(effect => {
      const val = Number(evaluateFormula(effect.value_formula, context))
      
      const apply = (statName) => {
        if (stats[statName] === undefined) return
        
        if (effect.modifier_type === 'add') {
          stats[statName] += val
        } else if (effect.modifier_type === 'multiply') {
          stats[statName] = Math.floor(stats[statName] * val)
        } else if (effect.modifier_type === 'percent_add') {
          // e.g. val = 0.5 (50%), so stats += base * 0.5
          // But which base? Current or Original? Usually Additive Percent relies on Base.
          // For simplicity, let's assume it modifies current value: val=10 (10%) -> * 1.1
          // Wait, user example: "decrease 1%, increase 1%". Formula returns 1.0 (1%).
          // If type is percent_add, and formula returns 1.0, does it mean +1% or +100%?
          // Let's assume formula returns the multiplier directly or percentage value?
          // User said: "increase 1%". Formula likely returns 0.01 or 1.
          // Let's assume standard: if result is 1, it means +1 (flat) or +1%?
          // "modifier_type" dropdown has "Percent Add (+%)".
          // If user enters 50, it means 50%. So we divide by 100.
          stats[statName] = Math.floor(stats[statName] * (1 + val / 100))
        }
      }
      
      if (effect.target_stat === 'all_stats') {
        ['strength', 'agility', 'intelligence', 'vitality', 'spirit', 'attack', 'phys_defense', 'mag_defense'].forEach(apply)
      } else if (effect.target_stat === 'all_attributes') { // str/agi/int/vit/spi
         ['strength', 'agility', 'intelligence', 'vitality', 'spirit'].forEach(apply)
      } else {
        // Map common names
        let target = effect.target_stat
        if (target === 'defense') target = 'phys_defense'
        apply(target)
      }
    })
  })
  
  return stats
})

const playerToMonsterDamage = computed(() => {
  if (!selectedPlayer.value || !selectedMonster.value) return null
  
  // Use FINAL stats
  const pStats = finalPlayerStats.value
  
  // Simple damage formula: (Atk - Def) * Multipliers
  let damage = pStats.attack - selectedMonster.value.defense
  if (damage < 0) damage = 0 

  // Multipliers
  let multiplier = 1.0
  if (isCrit.value) multiplier *= 1.5
  if (isFatal.value) multiplier *= 2.0

  // New formula: (Atk - Def) * Multiplier
  damage = Math.floor(damage * multiplier)
  
  // Percentage Reduction (damage_reduction is 0-100)
  const percentRed = selectedMonster.value.damage_reduction || 0
  damage = Math.floor(damage * (1.0 - percentRed / 100.0))

  // Fixed Reduction
  const fixedRed = selectedMonster.value.fixed_damage_reduction || 0
  damage = damage - fixedRed
  
  // Minimum damage can be 0 now as per requirement
  if (damage < 0) damage = 0
  
  return damage
})

const monsterToPlayerDamage = computed(() => {
  if (!selectedPlayer.value || !selectedMonster.value) return null
  
  // Use FINAL stats for player defense
  const pStats = finalPlayerStats.value

  // Monster Attack - Player Phys Defense
  // Assuming Monster does Physical damage for now.
  let damage = selectedMonster.value.attack - pStats.phys_defense
  if (damage < 0) damage = 0
  
  return damage
})

const loadData = async () => {
  if (!props.novelId) return
  try {
    players.value = await invoke('get_players', { novelId: props.novelId })
    monsters.value = await invoke('get_monsters', { novelId: props.novelId })
    buffs.value = await invoke('get_buffs', { novelId: props.novelId })
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
.result-card {
  text-align: center;
}
.damage-text {
  color: #f56c6c;
  font-size: 24px;
  margin: 0;
}
.card-header {
  font-weight: bold;
}
</style>
