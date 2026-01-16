<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('monster.add') }}</el-button>
    </div>
    <el-table :data="monsters" style="width: 100%" @row-dblclick="editMonster">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="level" :label="$t('common.level')" />
      <el-table-column prop="hp" :label="$t('player.hp')" />
      <el-table-column prop="attack" :label="$t('player.attack')" />
      <el-table-column :label="$t('common.actions')">
        <template #default="scope">
          <el-button size="small" @click="editMonster(scope.row)">{{ $t('common.edit') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showAddDialog" :title="$t('monster.add')" @keyup.enter="createMonster">
      <el-form :model="form">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createMonster">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditDialog" :title="$t('monster.edit')" width="70%" @keyup.enter="updateMonster">
      <el-form :model="editForm" label-width="120px">
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('common.name')"><el-input v-model="editForm.name" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('common.level')"><el-input-number v-model="editForm.level" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('monster.race')"><el-input v-model="editForm.race" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('monster.rarity')"><el-input v-model="editForm.rarity" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('monster.prefix')"><el-input v-model="editForm.prefix" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('player.hp')"><el-input-number v-model="editForm.hp" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('player.attack')"><el-input-number v-model="editForm.attack" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('monster.defense')"><el-input-number v-model="editForm.defense" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('monster.damage_reduction')"><el-input-number v-model="editForm.damage_reduction" /></el-form-item></el-col>
        </el-row>
        <el-form-item :label="$t('monster.description')">
          <el-input v-model="editForm.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showEditDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="updateMonster">{{ $t('common.save') }}</el-button>
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

const monsters = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const form = ref({ name: '' })
const editForm = ref({})

const loadMonsters = async () => {
  if (!props.novelId) return
  try {
    monsters.value = await invoke('get_monsters', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load monsters: ' + e)
  }
}

const createMonster = async () => {
  try {
    await invoke('create_monster', { novelId: props.novelId, name: form.value.name })
    showAddDialog.value = false
    form.value.name = ''
    loadMonsters()
    ElMessage.success('Monster created')
  } catch (e) {
    ElMessage.error('Failed to create monster: ' + e)
  }
}

const editMonster = (row) => {
  editForm.value = { ...row }
  showEditDialog.value = true
}

const updateMonster = async () => {
  try {
    await invoke('update_monster', {
      id: editForm.value.id,
      name: editForm.value.name,
      level: editForm.value.level,
      race: editForm.value.race,
      rarity: editForm.value.rarity,
      prefix: editForm.value.prefix,
      hp: editForm.value.hp,
      attack: editForm.value.attack,
      defense: editForm.value.defense,
      damageReduction: editForm.value.damage_reduction,
      drops: editForm.value.drops,
      description: editForm.value.description
    })
    showEditDialog.value = false
    loadMonsters()
    ElMessage.success('Monster updated')
  } catch (e) {
    ElMessage.error('Failed to update monster: ' + e)
  }
}

watch(() => props.novelId, loadMonsters)
onMounted(loadMonsters)
</script>
