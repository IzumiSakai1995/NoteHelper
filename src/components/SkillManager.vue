<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('skill.add') }}</el-button>
    </div>
    <el-table :data="skills" style="width: 100%" @row-dblclick="editSkill">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="level" :label="$t('skill.level')" />
      <el-table-column prop="description" :label="$t('common.description')" />
      <el-table-column :label="$t('common.actions')">
        <template #default="scope">
          <el-button size="small" @click="editSkill(scope.row)">{{ $t('common.edit') }}</el-button>
          <el-button size="small" type="danger" @click="deleteSkill(scope.row.id)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showAddDialog" :title="$t('skill.add')" @keyup.enter="createSkill">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="$t('common.name')"><el-input v-model="form.name" /></el-form-item>
        <el-form-item :label="$t('skill.level')"><el-input-number v-model="form.level" /></el-form-item>
        <el-form-item :label="$t('common.description')"><el-input v-model="form.description" type="textarea" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createSkill">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditDialog" :title="$t('skill.edit')" @keyup.enter="updateSkill">
      <el-form :model="editForm" label-width="100px">
        <el-form-item :label="$t('common.name')"><el-input v-model="editForm.name" /></el-form-item>
        <el-form-item :label="$t('skill.level')"><el-input-number v-model="editForm.level" /></el-form-item>
        <el-form-item :label="$t('common.description')"><el-input v-model="editForm.description" type="textarea" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showEditDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="updateSkill">{{ $t('common.save') }}</el-button>
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

const skills = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const form = ref({ name: '', description: '', level: 1 })
const editForm = ref({})

const loadSkills = async () => {
  if (!props.novelId) return
  try {
    skills.value = await invoke('get_skills', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load skills: ' + e)
  }
}

const createSkill = async () => {
  try {
    await invoke('create_skill', {
      novelId: props.novelId,
      name: form.value.name,
      description: form.value.description,
      level: form.value.level
    })
    showAddDialog.value = false
    form.value = { name: '', description: '', level: 1 }
    loadSkills()
    ElMessage.success('Skill created')
  } catch (e) {
    ElMessage.error('Failed to create skill: ' + e)
  }
}

const editSkill = (row) => {
  editForm.value = { ...row }
  showEditDialog.value = true
}

const updateSkill = async () => {
  try {
    await invoke('update_skill', {
      id: editForm.value.id,
      name: editForm.value.name,
      description: editForm.value.description,
      level: editForm.value.level
    })
    showEditDialog.value = false
    loadSkills()
    ElMessage.success('Skill saved')
  } catch (e) {
    ElMessage.error('Failed to save skill: ' + e)
  }
}

const deleteSkill = async (id) => {
  try {
    await invoke('delete_skill', { id })
    loadSkills()
    ElMessage.success('Skill deleted')
  } catch (e) {
    ElMessage.error('Failed to delete skill: ' + e)
  }
}

watch(() => props.novelId, () => loadSkills())
onMounted(() => loadSkills())
</script>

<style scoped>
</style>
