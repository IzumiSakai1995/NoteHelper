<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('template.add') }}</el-button>
    </div>
    <el-table :data="templates" style="width: 100%" @row-dblclick="editTemplate">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="level" :label="$t('common.level')" />
      <el-table-column prop="exp" label="EXP" />
      <el-table-column prop="max_hp" :label="$t('player.hp')" />
      <el-table-column prop="attack" :label="$t('player.attack')" />
      <el-table-column prop="phys_defense" :label="$t('player.phys_defense')" />
      <el-table-column prop="mag_defense" :label="$t('player.mag_defense')" />
      <el-table-column prop="crit_chance" :label="$t('player.crit_chance')" />
      <el-table-column prop="crit_dmg" :label="$t('player.crit_dmg')" />
      <el-table-column prop="deadly_chance" :label="$t('player.deadly_chance')" />
      <el-table-column prop="deadly_dmg" :label="$t('player.deadly_dmg')" />
      <el-table-column :label="$t('common.actions')">
        <template #default="scope">
          <el-button size="small" @click="editTemplate(scope.row)">{{ $t('common.edit') }}</el-button>
          <el-button size="small" type="danger" @click="deleteTemplate(scope.row.id)">{{ $t('common.delete') }}</el-button>
          <el-button size="small" type="success" @click="createPlayerFrom(scope.row)">{{ $t('template.create_from') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showAddDialog" :title="$t('template.add')" width="60%">
      <el-tabs v-model="activeTab">
        <el-tab-pane :label="$t('template.base_stats')" name="base">
          <el-form :model="form" label-width="110px">
            <el-form-item :label="$t('common.name')"><el-input v-model="form.name" /></el-form-item>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('common.level')"><el-input-number v-model="form.level" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item label="EXP"><el-input-number v-model="form.exp" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.strength')"><el-input-number v-model="form.strength" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.agility')"><el-input-number v-model="form.agility" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.intelligence')"><el-input-number v-model="form.intelligence" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.vitality')"><el-input-number v-model="form.vitality" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.spirit')"><el-input-number v-model="form.spirit" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.crit_chance')"><el-input-number v-model="form.crit_chance" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.crit_dmg')"><el-input-number v-model="form.crit_dmg" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.deadly_chance')"><el-input-number v-model="form.deadly_chance" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.deadly_dmg')"><el-input-number v-model="form.deadly_dmg" /></el-form-item></el-col>
            </el-row>
          </el-form>
        </el-tab-pane>
        <el-tab-pane :label="$t('template.coefficients')" name="coeffs">
          <el-form :model="form" label-width="220px">
            <el-form-item label="STR → Phys Attack"><el-input-number v-model="form.strength_to_phys_attack" :step="0.1" /></el-form-item>
            <el-form-item label="STR → Max HP"><el-input-number v-model="form.strength_to_max_hp" :step="0.1" /></el-form-item>
            <el-form-item label="AGI → Phys Attack"><el-input-number v-model="form.agility_to_phys_attack" :step="0.1" /></el-form-item>
            <el-form-item label="INT → Mag Attack"><el-input-number v-model="form.intelligence_to_mag_attack" :step="0.1" /></el-form-item>
            <el-form-item label="INT → Mag Defense"><el-input-number v-model="form.intelligence_to_mag_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Phys Defense"><el-input-number v-model="form.vitality_to_phys_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Mag Defense"><el-input-number v-model="form.vitality_to_mag_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Max HP"><el-input-number v-model="form.vitality_to_max_hp" :step="0.1" /></el-form-item>
            <el-form-item label="SPI → Mana Regen"><el-input-number v-model="form.spirit_to_mana_regen" :step="0.1" /></el-form-item>
            <el-form-item label="SPI → Mag Defense"><el-input-number v-model="form.spirit_to_mag_defense" :step="0.1" /></el-form-item>
          </el-form>
        </el-tab-pane>
        <el-tab-pane :label="$t('template.preview')" name="preview">
          <el-descriptions :column="2" border>
            <el-descriptions-item :label="$t('player.hp')">{{ preview.max_hp }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.attack')">{{ preview.attack }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.phys_defense')">{{ preview.phys_defense }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.mag_defense')">{{ preview.mag_defense }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.crit_chance')">{{ form.crit_chance }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.crit_dmg')">{{ form.crit_dmg }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.deadly_chance')">{{ form.deadly_chance }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.deadly_dmg')">{{ form.deadly_dmg }}%</el-descriptions-item>
          </el-descriptions>
        </el-tab-pane>
      </el-tabs>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createTemplate">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="showEditDialog" :title="$t('template.edit')" width="60%">
      <el-tabs v-model="activeEditTab">
        <el-tab-pane :label="$t('template.base_stats')" name="base">
          <el-form :model="editForm" label-width="110px">
            <el-form-item :label="$t('common.name')"><el-input v-model="editForm.name" /></el-form-item>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('common.level')"><el-input-number v-model="editForm.level" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item label="EXP"><el-input-number v-model="editForm.exp" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.strength')"><el-input-number v-model="editForm.strength" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.agility')"><el-input-number v-model="editForm.agility" /></el-form-item></el-col>
            </el-row>
            <el-row>
              <el-col :span="12"><el-form-item :label="$t('player.intelligence')"><el-input-number v-model="editForm.intelligence" /></el-form-item></el-col>
              <el-col :span="12"><el-form-item :label="$t('player.vitality')"><el-input-number v-model="editForm.vitality" /></el-form-item></el-col>
            </el-row>
            <el-row>
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
        <el-tab-pane :label="$t('template.coefficients')" name="coeffs">
          <el-form :model="editForm" label-width="220px">
            <el-form-item label="STR → Phys Attack"><el-input-number v-model="editForm.strength_to_phys_attack" :step="0.1" /></el-form-item>
            <el-form-item label="STR → Max HP"><el-input-number v-model="editForm.strength_to_max_hp" :step="0.1" /></el-form-item>
            <el-form-item label="AGI → Phys Attack"><el-input-number v-model="editForm.agility_to_phys_attack" :step="0.1" /></el-form-item>
            <el-form-item label="INT → Mag Attack"><el-input-number v-model="editForm.intelligence_to_mag_attack" :step="0.1" /></el-form-item>
            <el-form-item label="INT → Mag Defense"><el-input-number v-model="editForm.intelligence_to_mag_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Phys Defense"><el-input-number v-model="editForm.vitality_to_phys_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Mag Defense"><el-input-number v-model="editForm.vitality_to_mag_defense" :step="0.1" /></el-form-item>
            <el-form-item label="VIT → Max HP"><el-input-number v-model="editForm.vitality_to_max_hp" :step="0.1" /></el-form-item>
            <el-form-item label="SPI → Mana Regen"><el-input-number v-model="editForm.spirit_to_mana_regen" :step="0.1" /></el-form-item>
            <el-form-item label="SPI → Mag Defense"><el-input-number v-model="editForm.spirit_to_mag_defense" :step="0.1" /></el-form-item>
          </el-form>
        </el-tab-pane>
        <el-tab-pane :label="$t('template.preview')" name="preview">
          <el-descriptions :column="2" border>
            <el-descriptions-item :label="$t('player.hp')">{{ editPreview.max_hp }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.attack')">{{ editPreview.attack }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.phys_defense')">{{ editPreview.phys_defense }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.mag_defense')">{{ editPreview.mag_defense }}</el-descriptions-item>
            <el-descriptions-item :label="$t('player.crit_chance')">{{ editForm.crit_chance }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.crit_dmg')">{{ editForm.crit_dmg }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.deadly_chance')">{{ editForm.deadly_chance }}%</el-descriptions-item>
            <el-descriptions-item :label="$t('player.deadly_dmg')">{{ editForm.deadly_dmg }}%</el-descriptions-item>
          </el-descriptions>
        </el-tab-pane>
      </el-tabs>
      <template #footer>
        <el-button @click="showEditDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="updateTemplate">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

const props = defineProps({
  novelId: Number
})

const templates = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const activeTab = ref('base')
const activeEditTab = ref('base')

const defaultForm = () => ({
  name: '',
  level: 1,
  exp: 0,
  strength: 10,
  agility: 10,
  intelligence: 10,
  vitality: 10,
  spirit: 10,
  crit_chance: 0,
  crit_dmg: 0,
  deadly_chance: 0,
  deadly_dmg: 0,
  strength_to_phys_attack: 1.0,
  strength_to_max_hp: 0.0,
  agility_to_phys_attack: 1.0,
  intelligence_to_mag_attack: 1.0,
  intelligence_to_mag_defense: 0.0,
  vitality_to_phys_defense: 1.0,
  vitality_to_mag_defense: 0.5,
  vitality_to_max_hp: 10.0,
  spirit_to_mana_regen: 0.0,
  spirit_to_mag_defense: 0.5
})
const form = ref(defaultForm())
const editForm = ref({})

const preview = computed(() => {
  const f = form.value
  const strv = Number(f.strength) || 0
  const agiv = Number(f.agility) || 0
  const intv = Number(f.intelligence) || 0
  const vitv = Number(f.vitality) || 0
  const spiv = Number(f.spirit) || 0
  return {
    max_hp: Math.round(strv * f.strength_to_max_hp + vitv * f.vitality_to_max_hp),
    attack: Math.round(strv * f.strength_to_phys_attack + agiv * f.agility_to_phys_attack + intv * f.intelligence_to_mag_attack),
    phys_defense: Math.round(vitv * f.vitality_to_phys_defense),
    mag_defense: Math.round(intv * f.intelligence_to_mag_defense + spiv * f.spirit_to_mag_defense)
  }
})

const editPreview = computed(() => {
  const f = editForm.value
  const strv = Number(f.strength) || 0
  const agiv = Number(f.agility) || 0
  const intv = Number(f.intelligence) || 0
  const vitv = Number(f.vitality) || 0
  const spiv = Number(f.spirit) || 0
  return {
    max_hp: Math.round(strv * f.strength_to_max_hp + vitv * f.vitality_to_max_hp),
    attack: Math.round(strv * f.strength_to_phys_attack + agiv * f.agility_to_phys_attack + intv * f.intelligence_to_mag_attack),
    phys_defense: Math.round(vitv * f.vitality_to_phys_defense),
    mag_defense: Math.round(intv * f.intelligence_to_mag_defense + spiv * f.spirit_to_mag_defense)
  }
})

const loadTemplates = async () => {
  if (!props.novelId) return
  try {
    templates.value = await invoke('get_player_templates', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load templates: ' + e)
  }
}

const createTemplate = async () => {
  try {
    await invoke('create_player_template', {
      novelId: props.novelId,
      name: form.value.name,
      level: form.value.level,
      exp: form.value.exp,
      strength: form.value.strength,
      agility: form.value.agility,
      intelligence: form.value.intelligence,
      vitality: form.value.vitality,
      spirit: form.value.spirit,
      critChance: form.value.crit_chance,
      critDmg: form.value.crit_dmg,
      deadlyChance: form.value.deadly_chance,
      deadlyDmg: form.value.deadly_dmg,
      strengthToPhysAttack: form.value.strength_to_phys_attack,
      strengthToMaxHp: form.value.strength_to_max_hp,
      agilityToPhysAttack: form.value.agility_to_phys_attack,
      intelligenceToMagAttack: form.value.intelligence_to_mag_attack,
      intelligenceToMagDefense: form.value.intelligence_to_mag_defense,
      vitalityToPhysDefense: form.value.vitality_to_phys_defense,
      vitalityToMagDefense: form.value.vitality_to_mag_defense,
      vitalityToMaxHp: form.value.vitality_to_max_hp,
      spiritToManaRegen: form.value.spirit_to_mana_regen,
      spiritToMagDefense: form.value.spirit_to_mag_defense
    })
    showAddDialog.value = false
    form.value = defaultForm()
    await loadTemplates()
    ElMessage.success('Template created')
  } catch (e) {
    ElMessage.error('Failed to create template: ' + e)
  }
}

const editTemplate = (row) => {
  editForm.value = { ...row }
  showEditDialog.value = true
  activeEditTab.value = 'base'
}

const updateTemplate = async () => {
  try {
    await invoke('update_player_template', {
      id: editForm.value.id,
      name: editForm.value.name,
      level: editForm.value.level,
      exp: editForm.value.exp,
      strength: editForm.value.strength,
      agility: editForm.value.agility,
      intelligence: editForm.value.intelligence,
      vitality: editForm.value.vitality,
      spirit: editForm.value.spirit,
      critChance: editForm.value.crit_chance,
      critDmg: editForm.value.crit_dmg,
      deadlyChance: editForm.value.deadly_chance,
      deadlyDmg: editForm.value.deadly_dmg,
      strengthToPhysAttack: editForm.value.strength_to_phys_attack,
      strengthToMaxHp: editForm.value.strength_to_max_hp,
      agilityToPhysAttack: editForm.value.agility_to_phys_attack,
      intelligenceToMagAttack: editForm.value.intelligence_to_mag_attack,
      intelligenceToMagDefense: editForm.value.intelligence_to_mag_defense,
      vitalityToPhysDefense: editForm.value.vitality_to_phys_defense,
      vitalityToMagDefense: editForm.value.vitality_to_mag_defense,
      vitalityToMaxHp: editForm.value.vitality_to_max_hp,
      spiritToManaRegen: editForm.value.spirit_to_mana_regen,
      spiritToMagDefense: editForm.value.spirit_to_mag_defense
    })
    showEditDialog.value = false
    await loadTemplates()
    ElMessage.success('Template updated')
  } catch (e) {
    ElMessage.error('Failed to update template: ' + e)
  }
}

const deleteTemplate = async (id) => {
  try {
    await invoke('delete_player_template', { id })
    await loadTemplates()
    ElMessage.success('Template deleted')
  } catch (e) {
    ElMessage.error('Failed to delete template: ' + e)
  }
}

const createPlayerFrom = async (row) => {
  try {
    await invoke('create_player_from_template', {
      novelId: props.novelId,
      templateId: row.id,
      name: row.name
    })
    ElMessage.success('Player created from template')
  } catch (e) {
    ElMessage.error('Failed to create player: ' + e)
  }
}

watch(() => props.novelId, () => {
  loadTemplates()
})
onMounted(() => {
  loadTemplates()
})
</script>

<style scoped>
</style>
