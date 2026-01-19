<template>
  <div class="buff-manager">
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="openAddDialog">{{ $t('common.add') }} Buff</el-button>
    </div>

    <el-table :data="buffs" style="width: 100%">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="buff_type" :label="$t('common.type')" />
      <el-table-column prop="duration" :label="$t('common.duration')" />
      <el-table-column label="Effects">
        <template #default="scope">
          <div v-for="(effect, index) in (scope.row.effects || [])" :key="index" style="font-size: 12px">
            {{ effect.target_stat }}: {{ effect.value_formula }} ({{ effect.modifier_type }})
          </div>
        </template>
      </el-table-column>
      <el-table-column :label="$t('common.actions')" width="150">
        <template #default="scope">
          <el-button size="small" @click="openEditDialog(scope.row)">{{ $t('common.edit') }}</el-button>
          <el-button size="small" type="danger" @click="deleteBuff(scope.row.id)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showDialog" :title="isEdit ? 'Edit Buff' : 'Add Buff'" width="700px">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.type')">
          <el-select v-model="form.buff_type" placeholder="Select Type">
            <el-option label="Passive" value="passive" />
            <el-option label="Active" value="active" />
            <el-option label="Debuff" value="debuff" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('common.duration')">
          <el-input-number v-model="form.duration" :min="0" />
        </el-form-item>
        
        <el-divider>{{ $t('buff.dynamic_effects') }}</el-divider>
        <div v-for="(effect, index) in form.effects" :key="index" class="effect-row">
          <el-row :gutter="10">
            <el-col :span="6">
              <el-select v-model="effect.target_stat" :placeholder="$t('buff.target_stat')" size="small">
                <el-option :label="$t('player.attack')" value="attack" />
                <el-option :label="$t('player.defense')" value="defense" />
                <el-option :label="$t('player.hp')" value="hp" />
                <el-option :label="$t('player.stats')" value="all_stats" />
              </el-select>
            </el-col>
            <el-col :span="6">
              <el-select v-model="effect.modifier_type" :placeholder="$t('buff.modifier_type')" size="small">
                <el-option :label="$t('common.add') + ' (+)'" value="add" />
                <el-option label="Multiply (*)" value="multiply" />
                <el-option label="Percent Add (+%)" value="percent_add" />
              </el-select>
            </el-col>
            <el-col :span="10">
              <el-input v-model="effect.value_formula" :placeholder="$t('buff.formula') + ' (e.g. (1 - hp_pct) * 100)'" size="small" />
            </el-col>
            <el-col :span="2">
              <el-button type="danger" icon="Delete" circle size="small" @click="removeEffect(index)" />
            </el-col>
          </el-row>
        </div>
        <el-button type="dashed" style="width: 100%; margin-top: 10px" @click="addEffect">+ {{ $t('buff.add') }} {{ $t('buff.effect') }}</el-button>
        
        <div style="margin-top: 20px; font-size: 12px; color: #666;">
          {{ $t('buff.variables_hint') }}
        </div>
      </el-form>
      <template #footer>
        <el-button @click="showDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="submitForm">{{ isEdit ? $t('common.save') : $t('common.create') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

const props = defineProps({
  novelId: Number
})

const buffs = ref([])
const showDialog = ref(false)
const isEdit = ref(false)
const form = ref({
  id: null,
  name: '',
  buff_type: 'passive',
  duration: 0,
  effects: []
})

const loadBuffs = async () => {
  if (!props.novelId) return
  try {
    buffs.value = await invoke('get_buffs', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load buffs: ' + e)
  }
}

const openAddDialog = () => {
  isEdit.value = false
  form.value = {
    id: null,
    name: '',
    buff_type: 'passive',
    duration: 0,
    effects: []
  }
  showDialog.value = true
}

const openEditDialog = (row) => {
  isEdit.value = true
  form.value = {
    id: row.id,
    name: row.name,
    buff_type: row.buff_type || 'passive',
    duration: row.duration || 0,
    effects: row.effects ? JSON.parse(JSON.stringify(row.effects)) : []
  }
  showDialog.value = true
}

const addEffect = () => {
  form.value.effects.push({
    target_stat: 'attack',
    modifier_type: 'percent_add',
    value_formula: '0'
  })
}

const removeEffect = (index) => {
  form.value.effects.splice(index, 1)
}

const submitForm = async () => {
  try {
    if (isEdit.value) {
      await invoke('update_buff', {
        id: form.value.id,
        name: form.value.name,
        buffType: form.value.buff_type,
        duration: form.value.duration,
        attributes: null, // Legacy
        effects: form.value.effects
      })
      ElMessage.success('Buff updated')
    } else {
      await invoke('create_buff', {
        novelId: props.novelId,
        name: form.value.name,
        buffType: form.value.buff_type,
        duration: form.value.duration,
        attributes: null,
        effects: form.value.effects
      })
      ElMessage.success('Buff created')
    }
    showDialog.value = false
    loadBuffs()
  } catch (e) {
    ElMessage.error('Operation failed: ' + e)
  }
}

const deleteBuff = async (id) => {
  try {
    await invoke('delete_buff', { id })
    ElMessage.success('Buff deleted')
    loadBuffs()
  } catch (e) {
    ElMessage.error('Failed to delete buff: ' + e)
  }
}

watch(() => props.novelId, loadBuffs)
onMounted(loadBuffs)
</script>

<style scoped>
.effect-row {
  margin-bottom: 10px;
  padding: 10px;
  background-color: #f5f7fa;
  border-radius: 4px;
}
</style>
