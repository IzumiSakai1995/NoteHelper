<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('buff.add') }}</el-button>
    </div>
    <el-table :data="buffs" style="width: 100%" @row-dblclick="editBuff">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="buff_type" :label="$t('buff.type')" />
      <el-table-column prop="duration" :label="$t('buff.duration')" />
      <el-table-column :label="$t('buff.attributes')">
        <template #default="scope">
          <div v-if="scope.row.attributes">
            <el-tag v-for="(value, key) in scope.row.attributes" :key="key" size="small" style="margin-right: 5px">
              {{ getAttributeLabel(key) }}: {{ value }}
            </el-tag>
          </div>
        </template>
      </el-table-column>
      <el-table-column :label="$t('common.actions')" width="180">
        <template #default="scope">
          <el-button size="small" @click="editBuff(scope.row)">{{ $t('common.edit') }}</el-button>
          <el-button size="small" type="danger" @click="deleteBuff(scope.row.id)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- Add Dialog -->
    <el-dialog v-model="showAddDialog" :title="$t('buff.add')" @keyup.enter="createBuff">
      <el-form :model="form" label-width="120px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('buff.type')">
          <el-input v-model="form.buff_type" />
        </el-form-item>
        <el-form-item :label="$t('buff.duration')">
          <el-input-number v-model="form.duration" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createBuff">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit Dialog -->
    <el-dialog v-model="showEditDialog" :title="$t('buff.edit')" width="60%" @keyup.enter="updateBuff">
      <el-form :model="editForm" label-width="120px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="editForm.name" />
        </el-form-item>
        <el-form-item :label="$t('buff.type')">
          <el-input v-model="editForm.buff_type" />
        </el-form-item>
        <el-form-item :label="$t('buff.duration')">
          <el-input-number v-model="editForm.duration" />
        </el-form-item>
        
        <el-divider>{{ $t('buff.attributes') }}</el-divider>
        <el-row>
             <el-col :span="12"><el-form-item :label="$t('player.phys_attack')"><el-input-number v-model="editForm.attributes.phys_attack" /></el-form-item></el-col>
             <el-col :span="12"><el-form-item :label="$t('player.mag_attack')"><el-input-number v-model="editForm.attributes.mag_attack" /></el-form-item></el-col>
        </el-row>
        <el-row>
             <el-col :span="12"><el-form-item :label="$t('player.phys_defense')"><el-input-number v-model="editForm.attributes.phys_defense" /></el-form-item></el-col>
             <el-col :span="12"><el-form-item :label="$t('player.mag_defense')"><el-input-number v-model="editForm.attributes.mag_defense" /></el-form-item></el-col>
        </el-row>
        
        <el-row>
          <el-col :span="8"><el-form-item :label="$t('player.strength')"><el-input-number v-model="editForm.attributes.strength" /></el-form-item></el-col>
          <el-col :span="8"><el-form-item :label="$t('player.agility')"><el-input-number v-model="editForm.attributes.agility" /></el-form-item></el-col>
          <el-col :span="8"><el-form-item :label="$t('player.intelligence')"><el-input-number v-model="editForm.attributes.intelligence" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="8"><el-form-item :label="$t('player.vitality')"><el-input-number v-model="editForm.attributes.vitality" /></el-form-item></el-col>
          <el-col :span="8"><el-form-item :label="$t('player.spirit')"><el-input-number v-model="editForm.attributes.spirit" /></el-form-item></el-col>
        </el-row>

        <el-divider>{{ $t('item.custom_attributes') }}</el-divider>
        <div v-for="(value, key) in customAttributes" :key="key" style="margin-bottom: 10px; display: flex; align-items: center">
          <el-input v-model="attributeKeys[key]" :placeholder="$t('common.attribute_key')" style="width: 150px; margin-right: 10px" @change="updateAttributeKey(key, $event)" />
          <el-input-number v-model="editForm.attributes[key]" :placeholder="$t('common.attribute_value')" style="margin-right: 10px" />
          <el-button type="danger" icon="Delete" circle @click="removeAttribute(key)" />
        </div>
        <el-button type="primary" plain @click="addAttribute">{{ $t('common.add_custom_attribute') }}</el-button>
      </el-form>
      <template #footer>
        <el-button @click="showEditDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="updateBuff">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { Delete } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const props = defineProps({
  novelId: Number
})

const buffs = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const form = ref({ name: '', buff_type: '', duration: 0 })
const editForm = ref({ attributes: {} })
const attributeKeys = ref({}) 

const getAttributeLabel = (key) => {
   const map = {
     'phys_attack': t('player.phys_attack'),
     'mag_attack': t('player.mag_attack'),
     'phys_defense': t('player.phys_defense'),
     'mag_defense': t('player.mag_defense'),
     'strength': t('player.strength'),
     'agility': t('player.agility'),
     'intelligence': t('player.intelligence'),
     'vitality': t('player.vitality'),
     'spirit': t('player.spirit')
   }
   return map[key] || key
}

const standardKeys = [
  'phys_attack', 'mag_attack', 'phys_defense', 'mag_defense',
  'strength', 'agility', 'intelligence', 'vitality', 'spirit'
]

const customAttributes = computed(() => {
  if (!editForm.value.attributes) return {}
  const custom = {}
  for (const key in editForm.value.attributes) {
    if (!standardKeys.includes(key)) {
      custom[key] = editForm.value.attributes[key]
    }
  }
  return custom
})

const loadBuffs = async () => {
  if (!props.novelId) return
  try {
    buffs.value = await invoke('get_buffs', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load buffs: ' + e)
  }
}

const createBuff = async () => {
  if (!form.value.name) return
  try {
    await invoke('create_buff', { 
      novelId: props.novelId, 
      name: form.value.name,
      buffType: form.value.buff_type || null,
      duration: form.value.duration || 0,
      attributes: {}
    })
    showAddDialog.value = false
    form.value = { name: '', buff_type: '', duration: 0 }
    loadBuffs()
    ElMessage.success('Buff created')
  } catch (e) {
    ElMessage.error('Failed to create buff: ' + e)
  }
}

const editBuff = (row) => {
  editForm.value = JSON.parse(JSON.stringify(row))
  if (!editForm.value.attributes) {
    editForm.value.attributes = {}
  }
  
  attributeKeys.value = {}
  Object.keys(editForm.value.attributes).forEach(key => {
    if (!standardKeys.includes(key)) {
      attributeKeys.value[key] = key
    }
  })
  showEditDialog.value = true
}

const updateBuff = async () => {
  try {
    await invoke('update_buff', {
      id: editForm.value.id,
      name: editForm.value.name,
      buffType: editForm.value.buff_type,
      duration: editForm.value.duration,
      attributes: editForm.value.attributes
    })
    showEditDialog.value = false
    loadBuffs()
    ElMessage.success('Buff updated')
  } catch (e) {
    ElMessage.error('Failed to update buff: ' + e)
  }
}

const deleteBuff = async (id) => {
  try {
    await invoke('delete_buff', { id })
    loadBuffs()
    ElMessage.success('Buff deleted')
  } catch (e) {
    ElMessage.error('Failed to delete buff: ' + e)
  }
}

const addAttribute = () => {
  let counter = 1
  let newKey = `stat_${counter}`
  while (editForm.value.attributes[newKey] !== undefined) {
    counter++
    newKey = `stat_${counter}`
  }
  editForm.value.attributes[newKey] = 0
  attributeKeys.value[newKey] = newKey
}

const removeAttribute = (key) => {
  delete editForm.value.attributes[key]
  delete attributeKeys.value[key]
}

const updateAttributeKey = (oldKey, newKey) => {
  if (oldKey === newKey) return
  if (standardKeys.includes(newKey)) {
     ElMessage.warning('Cannot use standard attribute name')
     attributeKeys.value[oldKey] = oldKey
     return
  }
  if (editForm.value.attributes[newKey] !== undefined) {
    ElMessage.warning('Key already exists')
    attributeKeys.value[oldKey] = oldKey 
    return
  }
  const value = editForm.value.attributes[oldKey]
  delete editForm.value.attributes[oldKey]
  editForm.value.attributes[newKey] = value
  
  delete attributeKeys.value[oldKey]
  attributeKeys.value[newKey] = newKey
}

watch(() => props.novelId, loadBuffs)
onMounted(loadBuffs)
</script>
