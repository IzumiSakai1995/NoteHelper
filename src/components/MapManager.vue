<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="openAddDialog">{{ $t('map.add') }}</el-button>
    </div>
    <el-table :data="maps" style="width: 100%">
      <el-table-column prop="order_num" :label="$t('common.order')" width="80" />
      <el-table-column prop="name" :label="$t('common.name')" />
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

    <el-dialog v-model="showAddDialog" :title="isEdit ? $t('map.edit') : $t('map.add')" @keyup.enter="submitMap">
      <el-form :model="form" label-width="80px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.order')">
          <el-input-number v-model="form.order_num" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="submitMap">{{ isEdit ? $t('common.save') : $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showBindDialog" :title="$t('map.bind_monsters')" @keyup.enter="saveMapMonsters" width="800px">
      <div style="margin-bottom: 10px; display: flex; justify-content: flex-end;">
        <el-button type="success" size="small" @click="showAddMonsterDialog = true">
          {{ $t('monster.add') }}
        </el-button>
      </div>
      <el-transfer
        v-model="selectedMonsters"
        :data="allMonsters"
        :props="{ key: 'id', label: 'name' }"
        :titles="[$t('map.available'), $t('map.selected')]"
      />
      <template #footer>
        <el-button @click="showBindDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="saveMapMonsters">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>

    <!-- Quick Add Monster Dialog -->
    <el-dialog v-model="showAddMonsterDialog" :title="$t('monster.add')" width="400px" append-to-body>
      <el-form :model="monsterForm" label-width="80px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="monsterForm.name" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddMonsterDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createMonster">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

const props = defineProps({
  novelId: Number
})

const maps = ref([])
const allMonsters = ref([])
const showAddDialog = ref(false)
const showBindDialog = ref(false)
const showAddMonsterDialog = ref(false)
const isEdit = ref(false)
const form = ref({ id: null, name: '', order_num: 0 })
const monsterForm = ref({ name: '' })
const selectedMonsters = ref([])
const currentMapId = ref(null)

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
  form.value = { id: null, name: '', order_num: 0 }
  showAddDialog.value = true
}

const openEditDialog = (row) => {
  isEdit.value = true
  form.value = { id: row.id, name: row.name, order_num: row.order_num || 0 }
  showAddDialog.value = true
}

const submitMap = async () => {
  try {
    if (isEdit.value) {
      await invoke('update_map', {
        id: form.value.id,
        name: form.value.name,
        orderNum: form.value.order_num
      })
      ElMessage.success('Map updated')
    } else {
      await invoke('create_map', { 
        novelId: props.novelId, 
        name: form.value.name,
        orderNum: form.value.order_num 
      })
      ElMessage.success('Map created')
    }
    showAddDialog.value = false
    loadMaps()
  } catch (e) {
    ElMessage.error('Operation failed: ' + e)
  }
}

const createMonster = async () => {
  if (!monsterForm.value.name) return
  try {
    const id = await invoke('create_monster', { 
      novelId: props.novelId, 
      name: monsterForm.value.name 
    })
    
    ElMessage.success('Monster created')
    showAddMonsterDialog.value = false
    monsterForm.value.name = ''
    
    // Reload monsters and auto-select the new one
    await loadMonsters()
    selectedMonsters.value.push(id)
  } catch (e) {
    ElMessage.error('Failed to create monster: ' + e)
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
})
</script>
