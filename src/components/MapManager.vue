<template>
  <div>
    <!-- Main Map List -->
    <div v-if="!currentMap">
      <div style="margin-bottom: 10px">
        <el-button type="primary" @click="openAddDialog">{{ $t('map.add') }}</el-button>
      </div>
      <el-table 
        :data="mapTree" 
        style="width: 100%; height: calc(100vh - 200px);"
        row-key="id"
        default-expand-all
        height="100%"
        :tree-props="{ children: 'children', hasChildren: 'hasChildren' }"
        @row-dblclick="handleNavigate"
      >
        <el-table-column prop="order_num" :label="$t('common.order')" width="80" />
        <el-table-column prop="name" :label="$t('common.name')" />
        <el-table-column prop="description" :label="$t('monster.description')" show-overflow-tooltip />
        <el-table-column :label="$t('tabs.monsters')">
          <template #default="scope">
            <el-tag v-for="mId in (scope.row.monsters || [])" :key="mId" style="margin-right: 5px">
              {{ getMonsterName(mId) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column :label="$t('common.actions')">
          <template #default="scope">
            <el-button size="small" @click="openEditDialog(scope.row)">{{ $t('common.edit') }}</el-button>
            <el-button size="small" @click="openMonsterBinding(scope.row)">{{ $t('map.bind_monsters') }}</el-button>
            <el-button size="small" type="danger" @click="deleteMap(scope.row.id)">{{ $t('common.delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <!-- Map Detail View -->
    <MapDetail 
        v-else
        :mapData="currentMap"
        :subMaps="currentSubMaps"
        :mapMonsters="currentMapMonsters"
        :prefixes="prefixes"
        :ranks="ranks"
        @back="handleBack"
        @navigate="handleNavigateById"
        @edit-monster="openEditMonsterDialog"
        @bind-monster="handleBindMonster"
        @add-monster="handleAddMonster"
        @update-monster-inline="handleUpdateMonsterInline"
        @update-submap="handleUpdateSubMap"
      />

    <el-dialog v-model="showAddDialog" :title="isEdit ? $t('map.edit') : $t('map.add')" @keyup.enter="submitMap">
      <el-form :model="form" label-width="80px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.order')">
          <el-input-number v-model="form.order_num" />
        </el-form-item>
        <el-form-item label="父级地图">
          <el-select v-model="form.parent_id" placeholder="选择父级地图" clearable>
            <el-option
              v-for="map in flatMaps"
              :key="map.id"
              :label="map.name"
              :value="map.id"
              :disabled="map.id === form.id" 
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('monster.description')">
          <el-input v-model="form.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="submitMap">{{ isEdit ? $t('common.save') : $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showBindDialog" :title="$t('map.bind_monsters')" @keyup.enter="saveMapMonsters" width="800px">
      <div style="margin-bottom: 10px; display: flex; justify-content: flex-end;">
        <el-button type="success" size="small" @click="openAddMonsterDialog">
          {{ $t('monster.add') }}
        </el-button>
      </div>
      <el-transfer
        v-model="selectedMonsters"
        :data="allMonsters"
        :props="{ key: 'id', label: 'name' }"
        :titles="[$t('map.available'), $t('map.selected')]"
        filterable
        :filter-method="filterMethod"
      />
      <template #footer>
        <el-button @click="showBindDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveMapMonsters">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>

    <!-- Quick Add/Edit Monster Dialog -->
    <el-dialog v-model="showAddMonsterDialog" :title="isMonsterEdit ? $t('monster.edit') : $t('monster.add')" width="70%" append-to-body>
      <el-form :model="monsterForm" label-width="120px">
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('common.name')"><el-input v-model="monsterForm.name" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('common.level')"><el-input-number v-model="monsterForm.level" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="24"><el-form-item :label="$t('monster.race')"><el-input v-model="monsterForm.race" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="前缀 (Prefix)">
              <el-select v-model="monsterForm.prefix_id" clearable placeholder="Select Prefix">
                <el-option v-for="item in prefixes" :key="item.id" :label="item.name" :value="item.id">
                  <span style="float: left">{{ item.name }}</span>
                  <span style="float: right; color: #8492a6; font-size: 13px">
                    HP:x{{ item.hp_modifier }} Atk:x{{ item.attack_modifier }}
                  </span>
                </el-option>
              </el-select>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="等阶 (Rank)">
              <el-select v-model="monsterForm.rank_id" clearable placeholder="Select Rank">
                <el-option v-for="item in ranks" :key="item.id" :label="item.name" :value="item.id">
                   <span style="float: left">{{ item.name }}</span>
                  <span style="float: right; color: #8492a6; font-size: 13px">
                    HP:x{{ item.hp_modifier }} Atk:x{{ item.attack_modifier }}
                  </span>
                </el-option>
              </el-select>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="基础生命 (Base HP)">
              <el-input-number v-model="monsterForm.base_hp" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="基础攻击 (Base Atk)">
              <el-input-number v-model="monsterForm.base_attack" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="基础防御 (Base Def)">
              <el-input-number v-model="monsterForm.base_defense" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="$t('monster.damage_reduction') + ' (%)'">
              <el-input-number v-model="monsterForm.damage_reduction" :min="0" :max="100" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="固定减伤 (Fixed Red.)">
              <el-input-number v-model="monsterForm.fixed_damage_reduction" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="$t('monster.description')">
          <el-input v-model="monsterForm.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddMonsterDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveMonster">{{ isMonsterEdit ? $t('common.save') : $t('common.create') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import MapDetail from './MapDetail.vue'

const props = defineProps({
  novelId: Number
})

const maps = ref([])
const allMonsters = ref([])
const prefixes = ref([])
const ranks = ref([])
const showAddDialog = ref(false)
const showBindDialog = ref(false)
const showAddMonsterDialog = ref(false)
const isEdit = ref(false)
const isMonsterEdit = ref(false)
const editingMonsterId = ref(null)
const form = ref({ id: null, name: '', order_num: 0, parent_id: null, description: '' })
const monsterForm = ref({ 
  name: '', 
  level: 1, 
  race: '',
  prefix_id: null,
  rank_id: null,
  base_hp: 100,
  base_attack: 10,
  base_defense: 0,
  damage_reduction: 0,
  fixed_damage_reduction: 0,
  description: ''
})
const selectedMonsters = ref([])
const currentMapId = ref(null)
const navigationStack = ref([])

const currentMap = computed(() => {
  if (navigationStack.value.length === 0) return null
  const id = navigationStack.value[navigationStack.value.length - 1]
  return maps.value.find(m => m.id === id)
})

const currentSubMaps = computed(() => {
  if (!currentMap.value) return []
  return maps.value.filter(m => m.parent_id === currentMap.value.id)
})

const currentMapMonsters = computed(() => {
  if (!currentMap.value || !currentMap.value.monsters) return []
  return allMonsters.value.filter(m => (currentMap.value.monsters || []).includes(m.id))
})

const flatMaps = computed(() => maps.value)

const mapTree = computed(() => {
  const mapMap = new Map()
  const roots = []

  // Initialize map with empty children
  maps.value.forEach(m => {
    mapMap.set(m.id, { ...m, children: [] })
  })

  maps.value.forEach(m => {
    const node = mapMap.get(m.id)
    if (m.parent_id) {
      const parent = mapMap.get(m.parent_id)
      if (parent) {
        parent.children.push(node)
      } else {
        // Parent not found, treat as root
        roots.push(node)
      }
    } else {
      roots.push(node)
    }
  })

  return roots
})

const handleNavigate = (row) => {
  navigationStack.value.push(row.id)
}

const handleNavigateById = (id) => {
  navigationStack.value.push(id)
}

const handleBindMonster = (mapData) => {
  openMonsterBinding(mapData)
}

const handleAddMonster = (mapData) => {
  // Open Add Monster dialog, and set context so that after creation it binds to this map
  currentMapId.value = mapData.id
  // We need to signal that we want to bind to this map after creation
  // We can use a flag or check if showBindDialog is true (which it isn't here).
  // Let's use a specific state for this "Quick Add to Map" flow.
  
  // Actually, we can just open the Add Monster dialog, and in saveMonster, 
  // if currentMapId is set but showBindDialog is false, we know it's a direct add to map.
  
  openAddMonsterDialog()
}

const handleUpdateMonsterInline = async (updatedMonster) => {
  try {
    await invoke('update_monster', {
      id: updatedMonster.id,
      name: updatedMonster.name,
      level: updatedMonster.level,
      race: updatedMonster.race,
      prefixId: updatedMonster.prefix_id,
      rankId: updatedMonster.rank_id,
      baseHp: updatedMonster.base_hp,
      baseAttack: updatedMonster.base_attack,
      baseDefense: updatedMonster.base_defense,
      damageReduction: updatedMonster.damage_reduction,
      fixedDamageReduction: updatedMonster.fixed_damage_reduction,
      description: updatedMonster.description,
      rarity: updatedMonster.rarity,
      prefix: updatedMonster.prefix,
      drops: updatedMonster.drops
    })
    ElMessage.success('Monster updated')
    loadMonsters()
  } catch (e) {
    ElMessage.error('Failed to update monster: ' + e)
  }
}

const handleUpdateSubMap = async (updatedMap) => {
  try {
    await invoke('update_map', {
      id: updatedMap.id,
      name: updatedMap.name,
      orderNum: updatedMap.order_num,
      parentId: updatedMap.parent_id,
      description: updatedMap.description
    })
    ElMessage.success('Map updated')
    loadMaps()
  } catch (e) {
    ElMessage.error('Failed to update map: ' + e)
  }
}

const handleBack = () => {
  navigationStack.value.pop()
}

const openAddMonsterDialog = () => {
  isMonsterEdit.value = false
  editingMonsterId.value = null
  monsterForm.value = { 
    name: '', 
    level: 1, 
    race: '',
    prefix_id: null,
    rank_id: null,
    base_hp: 100,
    base_attack: 10,
    base_defense: 0,
    damage_reduction: 0,
    fixed_damage_reduction: 0,
    description: ''
  }
  showAddMonsterDialog.value = true
}

const openEditMonsterDialog = (monster) => {
  isMonsterEdit.value = true
  editingMonsterId.value = monster.id
  monsterForm.value = { 
    name: monster.name,
    level: monster.level,
    race: monster.race,
    prefix_id: monster.prefix_id,
    rank_id: monster.rank_id,
    base_hp: monster.base_hp,
    base_attack: monster.base_attack,
    base_defense: monster.base_defense,
    damage_reduction: monster.damage_reduction,
    fixed_damage_reduction: monster.fixed_damage_reduction,
    description: monster.description
  }
  showAddMonsterDialog.value = true
}

const loadMetadata = async () => {
  if (!props.novelId) return
  try {
    prefixes.value = await invoke('get_monster_prefixes', { novelId: props.novelId })
    ranks.value = await invoke('get_monster_ranks', { novelId: props.novelId })
  } catch (e) {
    console.error('Failed to load metadata', e)
  }
}

const loadMaps = async () => {
  if (!props.novelId) return
  try {
    maps.value = await invoke('get_maps', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load maps: ' + e)
  }
}

const loadMonsters = async () => {
  if (!props.novelId) return
  try {
    allMonsters.value = await invoke('get_monsters', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load monsters: ' + e)
  }
}

const openAddDialog = () => {
  isEdit.value = false
  form.value = { id: null, name: '', order_num: 0, parent_id: null, description: '' }
  showAddDialog.value = true
}

const openEditDialog = (row) => {
  isEdit.value = true
  form.value = { id: row.id, name: row.name, order_num: row.order_num || 0, parent_id: row.parent_id, description: row.description || '' }
  showAddDialog.value = true
}

const submitMap = async () => {
  try {
    if (isEdit.value) {
      await invoke('update_map', {
        id: form.value.id,
        name: form.value.name,
        orderNum: form.value.order_num,
        parentId: form.value.parent_id,
        description: form.value.description
      })
      ElMessage.success('Map updated')
    } else {
      await invoke('create_map', { 
        novelId: props.novelId, 
        name: form.value.name,
        orderNum: form.value.order_num,
        parentId: form.value.parent_id,
        description: form.value.description
      })
      ElMessage.success('Map created')
    }
    showAddDialog.value = false
    loadMaps()
  } catch (e) {
    ElMessage.error('Operation failed: ' + e)
  }
}

const saveMonster = async () => {
  if (!monsterForm.value.name) return
  
  try {
    if (isMonsterEdit.value) {
      await invoke('update_monster', {
        id: editingMonsterId.value,
        name: monsterForm.value.name,
        level: monsterForm.value.level,
        race: monsterForm.value.race,
        prefixId: monsterForm.value.prefix_id,
        rankId: monsterForm.value.rank_id,
        baseHp: monsterForm.value.base_hp,
        baseAttack: monsterForm.value.base_attack,
        baseDefense: monsterForm.value.base_defense,
        damageReduction: monsterForm.value.damage_reduction,
        fixedDamageReduction: monsterForm.value.fixed_damage_reduction,
        description: monsterForm.value.description,
        // Existing fields not in form
        rarity: null, 
        prefix: null,
        drops: null
      })
      ElMessage.success('Monster updated')
    } else {
      const id = await invoke('create_monster', { 
        novelId: props.novelId, 
        name: monsterForm.value.name,
        level: monsterForm.value.level,
        race: monsterForm.value.race,
        prefixId: monsterForm.value.prefix_id,
        rankId: monsterForm.value.rank_id,
        baseHp: monsterForm.value.base_hp,
        baseAttack: monsterForm.value.base_attack,
        baseDefense: monsterForm.value.base_defense,
        damageReduction: monsterForm.value.damage_reduction,
        fixedDamageReduction: monsterForm.value.fixed_damage_reduction,
        description: monsterForm.value.description
      })
      
      ElMessage.success('Monster created')
      
      // If we are in "Bind Monster" dialog (currentMapId is set and showBindDialog is true), add to list.
      // Or if we are directly adding to a map (currentMapId is set but showBindDialog is false)
      if (showBindDialog.value) {
        selectedMonsters.value.push(id)
      } else if (currentMapId.value) {
        // Direct add to map
        // We need to fetch current monsters of the map, add this one, and save
        const map = maps.value.find(m => m.id === currentMapId.value)
        if (map) {
          const currentMonsters = map.monsters || []
          // Check if already exists (shouldn't for new monster)
          if (!currentMonsters.includes(id)) {
             await invoke('update_map_monsters', { 
              id: currentMapId.value, 
              monsters: [...currentMonsters, id]
            })
            // Refresh maps to update view
            await loadMaps()
            ElMessage.success('Monster added to map')
          }
        }
      }
    }
    
    showAddMonsterDialog.value = false
    
    // Reset form
    monsterForm.value = { 
      name: '', 
      level: 1, 
      race: '',
      prefix_id: null,
      rank_id: null,
      base_hp: 100,
      base_attack: 10,
      base_defense: 0,
      damage_reduction: 0,
      fixed_damage_reduction: 0,
      description: ''
    }
    isMonsterEdit.value = false
    editingMonsterId.value = null
    
    await loadMonsters()
  } catch (e) {
    ElMessage.error('Failed to save monster: ' + e)
  }
}

const openMonsterBinding = (row) => {
  currentMapId.value = row.id
  selectedMonsters.value = row.monsters || []
  showBindDialog.value = true
}

const saveMapMonsters = async () => {
  try {
    await invoke('update_map_monsters', { 
      id: currentMapId.value, 
      monsters: selectedMonsters.value 
    })
    showBindDialog.value = false
    loadMaps()
    ElMessage.success('Map updated')
  } catch (e) {
    ElMessage.error('Failed to update map: ' + e)
  }
}

const getMonsterName = (id) => {
  const m = allMonsters.value.find(m => m.id === id)
  return m ? m.name : id
}

const filterMethod = (query, item) => {
  return item.name.toLowerCase().includes(query.toLowerCase())
}

const deleteMap = async (id) => {
  try {
    await invoke('delete_map', { id })
    loadMaps()
    ElMessage.success('Map deleted')
  } catch (e) {
    ElMessage.error('Failed to delete map: ' + e)
  }
}

watch(() => props.novelId, () => {
  loadMaps()
  loadMonsters()
})

onMounted(() => {
  loadMaps()
  loadMonsters()
  loadMetadata()
})
</script>
