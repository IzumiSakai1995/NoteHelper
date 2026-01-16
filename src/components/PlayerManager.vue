<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('player.add') }}</el-button>
    </div>
    <el-table :data="players" style="width: 100%" @row-dblclick="editPlayer">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="level" :label="$t('common.level')" />
      <el-table-column prop="hp" :label="$t('player.hp')" />
      <el-table-column prop="shield" :label="$t('player.shield')" />
      <el-table-column prop="attack" :label="$t('player.attack')" />
      <el-table-column :label="$t('common.actions')">
        <template #default="scope">
          <el-button size="small" @click="editPlayer(scope.row)">{{ $t('common.edit') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- Add Dialog -->
    <el-dialog v-model="showAddDialog" :title="$t('player.add')" @keyup.enter="createPlayer">
      <el-form :model="form">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createPlayer">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit Dialog (Stats & Equipment) -->
    <el-dialog v-model="showEditDialog" :title="$t('player.edit')" width="70%" @keyup.enter="updatePlayer">
      <el-tabs v-model="activeTab">
        <el-tab-pane :label="$t('player.stats')" name="stats">
          <!-- Total Attributes Summary -->
          <div class="total-stats" style="margin-bottom: 20px">
            <el-descriptions :title="$t('player.total_attributes')" :column="2" border>
              <el-descriptions-item :label="$t('player.hp')">
                {{ totalStats.hp }}
                <span v-if="equipmentStats.hp > 0" class="equip-bonus">(Eq: +{{ equipmentStats.hp }})</span>
                <span v-if="buffStats.hp > 0" class="buff-bonus">(Buff: +{{ buffStats.hp }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.shield')">
                {{ totalStats.shield }}
                <span v-if="equipmentStats.shield > 0" class="equip-bonus">(Eq: +{{ equipmentStats.shield }})</span>
                <span v-if="buffStats.shield > 0" class="buff-bonus">(Buff: +{{ buffStats.shield }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.attack')">
                {{ totalStats.attack }}
                <span v-if="equipmentStats.attack > 0" class="equip-bonus">(Eq: +{{ equipmentStats.attack }})</span>
                <span v-if="buffStats.attack > 0" class="buff-bonus">(Buff: +{{ buffStats.attack }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.phys_defense')">
                {{ totalStats.phys_defense }}
                <span v-if="equipmentStats.phys_defense > 0" class="equip-bonus">(Eq: +{{ equipmentStats.phys_defense }})</span>
                <span v-if="buffStats.phys_defense > 0" class="buff-bonus">(Buff: +{{ buffStats.phys_defense }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.mag_defense')">
                {{ totalStats.mag_defense }}
                <span v-if="equipmentStats.mag_defense > 0" class="equip-bonus">(Eq: +{{ equipmentStats.mag_defense }})</span>
                <span v-if="buffStats.mag_defense > 0" class="buff-bonus">(Buff: +{{ buffStats.mag_defense }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.strength')">
                {{ totalStats.strength }}
                <span v-if="equipmentStats.strength > 0" class="equip-bonus">(Eq: +{{ equipmentStats.strength }})</span>
                <span v-if="buffStats.strength > 0" class="buff-bonus">(Buff: +{{ buffStats.strength }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.agility')">
                {{ totalStats.agility }}
                <span v-if="equipmentStats.agility > 0" class="equip-bonus">(Eq: +{{ equipmentStats.agility }})</span>
                <span v-if="buffStats.agility > 0" class="buff-bonus">(Buff: +{{ buffStats.agility }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.intelligence')">
                {{ totalStats.intelligence }}
                <span v-if="equipmentStats.intelligence > 0" class="equip-bonus">(Eq: +{{ equipmentStats.intelligence }})</span>
                <span v-if="buffStats.intelligence > 0" class="buff-bonus">(Buff: +{{ buffStats.intelligence }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.vitality')">
                {{ totalStats.vitality }}
                <span v-if="equipmentStats.vitality > 0" class="equip-bonus">(Eq: +{{ equipmentStats.vitality }})</span>
                <span v-if="buffStats.vitality > 0" class="buff-bonus">(Buff: +{{ buffStats.vitality }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.spirit')">
                {{ totalStats.spirit }}
                <span v-if="equipmentStats.spirit > 0" class="equip-bonus">(Eq: +{{ equipmentStats.spirit }})</span>
                <span v-if="buffStats.spirit > 0" class="buff-bonus">(Buff: +{{ buffStats.spirit }})</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.crit_chance')">
                {{ totalStats.crit_chance }}%
                <span v-if="equipmentStats.crit_chance > 0" class="equip-bonus">(Eq: +{{ equipmentStats.crit_chance }}%)</span>
                <span v-if="buffStats.crit_chance > 0" class="buff-bonus">(Buff: +{{ buffStats.crit_chance }}%)</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.crit_dmg')">
                {{ totalStats.crit_dmg }}%
                <span v-if="equipmentStats.crit_dmg > 0" class="equip-bonus">(Eq: +{{ equipmentStats.crit_dmg }}%)</span>
                <span v-if="buffStats.crit_dmg > 0" class="buff-bonus">(Buff: +{{ buffStats.crit_dmg }}%)</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.deadly_chance')">
                {{ totalStats.deadly_chance }}%
                <span v-if="equipmentStats.deadly_chance > 0" class="equip-bonus">(Eq: +{{ equipmentStats.deadly_chance }}%)</span>
                <span v-if="buffStats.deadly_chance > 0" class="buff-bonus">(Buff: +{{ buffStats.deadly_chance }}%)</span>
              </el-descriptions-item>
              <el-descriptions-item :label="$t('player.deadly_dmg')">
                {{ totalStats.deadly_dmg }}%
                <span v-if="equipmentStats.deadly_dmg > 0" class="equip-bonus">(Eq: +{{ equipmentStats.deadly_dmg }}%)</span>
                <span v-if="buffStats.deadly_dmg > 0" class="buff-bonus">(Buff: +{{ buffStats.deadly_dmg }}%)</span>
              </el-descriptions-item>
            </el-descriptions>

            <div v-if="Object.keys(equipmentStats.custom).length > 0" style="margin-top: 10px">
              <strong>{{ $t('player.extra_stats') }}: </strong>
              <el-tag v-for="(val, key) in equipmentStats.custom" :key="key" type="success" style="margin-right: 5px">
                {{ key }}: +{{ val }}
              </el-tag>
            </div>
            <div v-if="Object.keys(buffStats.custom).length > 0" style="margin-top: 10px">
              <strong>{{ $t('player.buff_extra_stats') }}: </strong>
              <el-tag v-for="(val, key) in buffStats.custom" :key="key" type="warning" style="margin-right: 5px">
                {{ key }}: +{{ val }}
              </el-tag>
            </div>
          </div>
          
          <el-divider>{{ $t('player.base_stats_editing') }}</el-divider>

          <el-form :model="editForm" label-width="100px">
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.hp')"><el-input-number v-model="editForm.hp" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.shield')"><el-input-number v-model="editForm.shield" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.attack')"><el-input-number v-model="editForm.attack" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.phys_defense')"><el-input-number v-model="editForm.phys_defense" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.mag_defense')"><el-input-number v-model="editForm.mag_defense" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.strength')"><el-input-number v-model="editForm.strength" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.agility')"><el-input-number v-model="editForm.agility" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.intelligence')"><el-input-number v-model="editForm.intelligence" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.vitality')"><el-input-number v-model="editForm.vitality" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.spirit')"><el-input-number v-model="editForm.spirit" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.crit_chance')"><el-input-number v-model="editForm.crit_chance" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.crit_dmg')"><el-input-number v-model="editForm.crit_dmg" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.deadly_chance')"><el-input-number v-model="editForm.deadly_chance" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.deadly_dmg')"><el-input-number v-model="editForm.deadly_dmg" /></el-form-item></el-col>
            </el-row>
          </el-form>
        </el-tab-pane>
        <el-tab-pane :label="$t('player.equipment')" name="equipment">
          <el-form :model="equipmentForm" label-width="120px">
            <el-row v-for="(slots, rowIndex) in equipmentRows" :key="rowIndex">
              <el-col :span="12" v-for="slot in slots" :key="slot.key">
                <el-form-item :label="slot.label">
                  <el-select v-model="equipmentForm[slot.key]" clearable placeholder="Select Item" @change="onEquipmentChange(slot.key)">
                    <el-option v-for="item in getItemsForSlot(slot.type)" :key="item.id" :label="item.name" :value="item.id" />
                  </el-select>
                </el-form-item>
              </el-col>
            </el-row>
          </el-form>
        </el-tab-pane>
        <el-tab-pane :label="$t('tabs.buffs')" name="buffs">
           <div style="margin-bottom: 10px">
             <el-button type="primary" size="small" @click="showApplyBuffDialog = true">{{ $t('buff.apply') }}</el-button>
           </div>
           <el-table :data="activeBuffs" style="width: 100%">
              <el-table-column prop="name" :label="$t('common.name')" />
              <el-table-column :label="$t('buff.remaining')">
                <template #default="scope">
                  {{ getRemainingTime(scope.row) }}s
                </template>
              </el-table-column>
              <el-table-column :label="$t('buff.attributes')">
               <template #default="scope">
                 <div v-if="scope.row.attributes">
                   <el-tag v-for="(value, key) in scope.row.attributes" :key="key" size="small" style="margin-right: 5px">
                     {{ key }}: {{ value }}
                   </el-tag>
                 </div>
               </template>
             </el-table-column>
             <el-table-column :label="$t('common.actions')">
               <template #default="scope">
                 <el-button size="small" type="danger" @click="removeBuff(scope.row.id)">{{ $t('buff.remove') }}</el-button>
               </template>
             </el-table-column>
           </el-table>
        </el-tab-pane>
      </el-tabs>

      <!-- Apply Buff Dialog -->
      <el-dialog v-model="showApplyBuffDialog" :title="$t('buff.select_buff')" width="40%" append-to-body>
        <el-table :data="availableBuffs" @row-click="applyBuff">
          <el-table-column prop="name" :label="$t('common.name')" />
          <el-table-column prop="duration" :label="$t('buff.duration')" />
          <el-table-column :label="$t('buff.attributes')">
             <template #default="scope">
               <div v-if="scope.row.attributes">
                 <el-tag v-for="(value, key) in scope.row.attributes" :key="key" size="small" style="margin-right: 5px">
                   {{ key }}: {{ value }}
                 </el-tag>
               </div>
             </template>
          </el-table-column>
          <el-table-column label="">
            <template #default="scope">
              <el-button size="small" type="primary">{{ $t('buff.apply') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-dialog>
      <template #footer>
        <el-button @click="showEditDialog = false">Cancel</el-button>
        <el-button type="primary" @click="saveAll">Save</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

const props = defineProps({
  novelId: Number
})

const players = ref([])
const items = ref([])
const activeBuffs = ref([])
const availableBuffs = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const showApplyBuffDialog = ref(false)
const activeTab = ref('stats')
const form = ref({ name: '' })
const editForm = ref({})
const equipmentForm = ref({})

const equipmentSlots = [
  { key: 'main_hand', label: 'Main Hand', type: ['Main Hand', 'Two-Handed'] },
  { key: 'off_hand', label: 'Off Hand', type: ['Off Hand'] },
  { key: 'chest', label: 'Chest', type: ['Chest'] },
  { key: 'legs', label: 'Legs', type: ['Legs'] },
  { key: 'hands', label: 'Hands', type: ['Hands'] },
  { key: 'feet', label: 'Feet', type: ['Feet'] },
  { key: 'ring1', label: 'Ring 1', type: ['Ring'] },
  { key: 'ring2', label: 'Ring 2', type: ['Ring'] },
  { key: 'necklace', label: 'Necklace', type: ['Necklace'] },
  { key: 'earring', label: 'Earring', type: ['Earring'] },
  { key: 'belt', label: 'Belt', type: ['Belt'] },
  { key: 'back', label: 'Back', type: ['Back'] }
]

const equipmentRows = []
for (let i = 0; i < equipmentSlots.length; i += 2) {
  equipmentRows.push(equipmentSlots.slice(i, i + 2))
}

const loadPlayers = async () => {
  if (!props.novelId) return
  try {
    players.value = await invoke('get_players', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load players: ' + e)
  }
}

const loadItems = async () => {
  if (!props.novelId) return
  try {
    items.value = await invoke('get_items', { novelId: props.novelId })
  } catch (e) {
    console.error(e)
  }
}

const loadAvailableBuffs = async () => {
  if (!props.novelId) return
  try {
    availableBuffs.value = await invoke('get_buffs', { novelId: props.novelId })
  } catch (e) {
    console.error(e)
  }
}

const loadPlayerBuffs = async () => {
  if (!editForm.value.id) return
  try {
    activeBuffs.value = await invoke('get_player_buffs', { playerId: editForm.value.id })
  } catch (e) {
    console.error(e)
  }
}

const getItemsForSlot = (types) => {
  return items.value.filter(item => types.includes(item.item_type))
}

const getRemainingTime = (buff) => {
  if (!buff.duration) return '∞'
  const applied = new Date(buff.applied_at).getTime()
  // applied_at from sqlx might be UTC or Local. 
  // NaiveDateTime from sqlx is usually treated as is.
  // Since we used LOCALTIMESTAMP in query, we should treat it as local.
  // But browser Date treats 'YYYY-MM-DDTHH:mm:ss' as local if no Z.
  // sqlx usually returns ISO string without Z.
  
  const now = Date.now()
  // Adjust for timezone offset if needed, but let's assume they match for now.
  // Actually, NaiveDateTime string might be interpreted differently.
  // Let's rely on simple diff.
  
  // Actually, simpler: backend filtered it. So it IS active.
  // Just show calculate:
  const elapsed = (now - applied) / 1000
  const remaining = Math.max(0, Math.round(buff.duration - elapsed))
  return remaining
}

const createPlayer = async () => {
  try {
    await invoke('create_player', { novelId: props.novelId, name: form.value.name })
    showAddDialog.value = false
    form.value.name = ''
    loadPlayers()
    ElMessage.success('Player created')
  } catch (e) {
    ElMessage.error('Failed to create player: ' + e)
  }
}

const editPlayer = (row) => {
  editForm.value = { ...row }
  equipmentForm.value = row.equipment || {}
  activeTab.value = 'stats'
  showEditDialog.value = true
  loadItems() 
  loadAvailableBuffs()
  loadPlayerBuffs()
}

const onEquipmentChange = (slotKey) => {
  if (slotKey === 'main_hand') {
    const itemId = equipmentForm.value.main_hand
    const item = items.value.find(i => i.id === itemId)
    if (item && item.item_type === 'Two-Handed') {
      equipmentForm.value.off_hand = null 
      ElMessage.info('Equipped Two-Handed weapon, Off Hand unequipped')
    }
  } else if (slotKey === 'off_hand') {
    const mainHandId = equipmentForm.value.main_hand
    const mainHandItem = items.value.find(i => i.id === mainHandId)
    if (mainHandItem && mainHandItem.item_type === 'Two-Handed') {
      equipmentForm.value.off_hand = null 
      ElMessage.warning('Cannot equip Off Hand with Two-Handed weapon')
    }
  }
}

const applyBuff = async (row) => {
  if (!editForm.value.id) return
  try {
    await invoke('apply_buff', { playerId: editForm.value.id, buffId: row.id })
    ElMessage.success('Buff applied')
    showApplyBuffDialog.value = false
    loadPlayerBuffs()
  } catch (e) {
    ElMessage.error('Failed to apply buff: ' + e)
  }
}

const removeBuff = async (id) => {
  try {
    await invoke('remove_buff', { id })
    ElMessage.success('Buff removed')
    loadPlayerBuffs()
  } catch (e) {
    ElMessage.error('Failed to remove buff: ' + e)
  }
}

// Calculate equipment stats
const equipmentStats = computed(() => {
  const stats = {
    hp: 0, shield: 0, attack: 0, phys_defense: 0, mag_defense: 0,
    strength: 0, agility: 0, intelligence: 0,
    vitality: 0, spirit: 0,
    crit_chance: 0, crit_dmg: 0, deadly_chance: 0, deadly_dmg: 0,
    custom: {}
  }

  for (const key in equipmentForm.value) {
    const itemId = equipmentForm.value[key]
    if (!itemId) continue
    const item = items.value.find(i => i.id === itemId)
    if (item && item.attributes) {
      for (const attrKey in item.attributes) {
        const val = Number(item.attributes[attrKey]) || 0
        if (stats[attrKey] !== undefined) {
          stats[attrKey] += val
        } else if (attrKey === 'phys_attack' || attrKey === 'mag_attack') {
             stats.attack += val 
        } else {
          stats.custom[attrKey] = (stats.custom[attrKey] || 0) + val
        }
      }
    }
  }
  return stats
})

const buffStats = computed(() => {
  const stats = {
    hp: 0, shield: 0, attack: 0, phys_defense: 0, mag_defense: 0,
    strength: 0, agility: 0, intelligence: 0,
    vitality: 0, spirit: 0,
    crit_chance: 0, crit_dmg: 0, deadly_chance: 0, deadly_dmg: 0,
    custom: {}
  }

  activeBuffs.value.forEach(buff => {
    if (buff.attributes) {
      for (const key in buff.attributes) {
        const val = Number(buff.attributes[key]) || 0
        if (stats[key] !== undefined) {
          stats[key] += val
        } else if (key === 'phys_attack' || key === 'mag_attack') {
          stats.attack += val
        } else {
          stats.custom[key] = (stats.custom[key] || 0) + val
        }
      }
    }
  })
  return stats
})

// Calculate total stats
const totalStats = computed(() => {
  const base = editForm.value
  const equip = equipmentStats.value
  const buff = buffStats.value
  return {
    hp: (base.hp || 0) + equip.hp + buff.hp,
    shield: (base.shield || 0) + equip.shield + buff.shield,
    attack: (base.attack || 0) + equip.attack + buff.attack,
    phys_defense: (base.phys_defense || 0) + equip.phys_defense + buff.phys_defense,
    mag_defense: (base.mag_defense || 0) + equip.mag_defense + buff.mag_defense,
    strength: (base.strength || 0) + equip.strength + buff.strength,
    agility: (base.agility || 0) + equip.agility + buff.agility,
    intelligence: (base.intelligence || 0) + equip.intelligence + buff.intelligence,
    vitality: (base.vitality || 0) + equip.vitality + buff.vitality,
    spirit: (base.spirit || 0) + equip.spirit + buff.spirit,
    crit_chance: (base.crit_chance || 0) + equip.crit_chance + buff.crit_chance,
    crit_dmg: (base.crit_dmg || 0) + equip.crit_dmg + buff.crit_dmg,
    deadly_chance: (base.deadly_chance || 0) + equip.deadly_chance + buff.deadly_chance,
    deadly_dmg: (base.deadly_dmg || 0) + equip.deadly_dmg + buff.deadly_dmg
  }
})

const updatePlayer = async () => {
  try {
    await invoke('update_player_stats', {
      id: editForm.value.id,
      hp: editForm.value.hp,
      shield: editForm.value.shield,
      attack: editForm.value.attack,
      physDefense: editForm.value.phys_defense,
      magDefense: editForm.value.mag_defense,
      strength: editForm.value.strength,
      agility: editForm.value.agility,
      intelligence: editForm.value.intelligence,
      vitality: editForm.value.vitality,
      spirit: editForm.value.spirit,
      critChance: editForm.value.crit_chance,
      critDmg: editForm.value.crit_dmg,
      deadlyChance: editForm.value.deadly_chance,
      deadlyDmg: editForm.value.deadly_dmg
    })
  } catch (e) {
    throw new Error('Failed to update stats: ' + e)
  }
}

const updateEquipment = async () => {
  try {
    await invoke('update_player_equipment', {
      id: editForm.value.id,
      equipment: equipmentForm.value
    })
  } catch (e) {
    throw new Error('Failed to update equipment: ' + e)
  }
}

const saveAll = async () => {
  try {
    await updatePlayer()
    await updateEquipment()
    showEditDialog.value = false
    loadPlayers()
    ElMessage.success('Player saved')
  } catch (e) {
    ElMessage.error(e.message)
  }
}

watch(() => props.novelId, () => {
  loadPlayers()
  loadItems()
})
onMounted(() => {
  loadPlayers()
  loadItems()
})
</script>

<style scoped>
.equip-bonus {
  color: #67c23a;
  margin-left: 5px;
  font-weight: bold;
}
.buff-bonus {
  color: #e6a23c;
  margin-left: 5px;
  font-weight: bold;
}
.total-stats {
  padding: 10px;
}
</style>
