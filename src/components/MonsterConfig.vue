<template>
  <div class="monster-config">
    <el-tabs v-model="activeTab" @tab-click="handleTabClick">
      <!-- Prefixes Tab -->
      <el-tab-pane label="怪物前缀 (Prefixes)" name="prefixes">
        <div class="tab-content">
          <div class="header">
            <el-button type="primary" @click="openPrefixDialog()">新增前缀 (Add Prefix)</el-button>
          </div>
          
          <el-table :data="prefixes" style="width: 100%" v-loading="loadingPrefixes">
            <el-table-column prop="name" label="名称 (Name)" width="180" />
            <el-table-column prop="hp_modifier" label="生命系数 (HP Mod)" width="150" />
            <el-table-column prop="attack_modifier" label="攻击系数 (Atk Mod)" width="150" />
            <el-table-column prop="defense_modifier" label="防御系数 (Def Mod)" width="150" />
            <el-table-column label="操作 (Actions)">
              <template #default="scope">
                <el-button size="small" @click="openPrefixDialog(scope.row)">编辑 (Edit)</el-button>
                <el-button size="small" type="danger" @click="deletePrefix(scope.row.id)">删除 (Delete)</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-tab-pane>

      <!-- Ranks Tab -->
      <el-tab-pane label="怪物等阶 (Ranks)" name="ranks">
        <div class="tab-content">
          <div class="header">
            <el-button type="primary" @click="openRankDialog()">新增等阶 (Add Rank)</el-button>
          </div>
          
          <el-table :data="ranks" style="width: 100%" v-loading="loadingRanks">
            <el-table-column prop="name" label="名称 (Name)" width="180">
              <template #default="scope">
                <span :style="{ color: scope.row.color || '#606266' }">{{ scope.row.name }}</span>
              </template>
            </el-table-column>
            <el-table-column prop="hp_modifier" label="生命系数 (HP Mod)" width="150" />
            <el-table-column prop="attack_modifier" label="攻击系数 (Atk Mod)" width="150" />
            <el-table-column prop="defense_modifier" label="防御系数 (Def Mod)" width="150" />
            <el-table-column label="颜色 (Color)" width="100">
              <template #default="scope">
                <div :style="{ width: '20px', height: '20px', backgroundColor: scope.row.color, borderRadius: '4px', border: '1px solid #dcdfe6' }"></div>
              </template>
            </el-table-column>
            <el-table-column label="操作 (Actions)">
              <template #default="scope">
                <el-button size="small" @click="openRankDialog(scope.row)">编辑 (Edit)</el-button>
                <el-button size="small" type="danger" @click="deleteRank(scope.row.id)">删除 (Delete)</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-tab-pane>

      <!-- Settings Tab -->
      <el-tab-pane label="全局设置 (Settings)" name="settings">
        <div class="tab-content form-content">
          <el-form :model="settings" label-width="180px" v-loading="loadingSettings">
            <el-form-item label="等级属性系数 (Level Coeff)">
              <el-input-number v-model="settings.level_coefficient" :precision="2" :step="0.1" />
              <div class="help-text">怪物属性随等级增长的系数 (Global multiplier for level scaling)</div>
            </el-form-item>
            <el-form-item>
              <el-button type="primary" @click="saveSettings">保存设置 (Save)</el-button>
            </el-form-item>
          </el-form>
        </div>
      </el-tab-pane>
    </el-tabs>

    <!-- Prefix Dialog -->
    <el-dialog v-model="prefixDialogVisible" :title="editingPrefix ? '编辑前缀' : '新增前缀'" width="500px">
      <el-form :model="prefixForm" label-width="120px">
        <el-form-item label="名称 (Name)">
          <el-input v-model="prefixForm.name" />
        </el-form-item>
        <el-form-item label="生命系数">
          <el-input-number v-model="prefixForm.hp_modifier" :precision="2" :step="0.1" />
        </el-form-item>
        <el-form-item label="攻击系数">
          <el-input-number v-model="prefixForm.attack_modifier" :precision="2" :step="0.1" />
        </el-form-item>
        <el-form-item label="防御系数">
          <el-input-number v-model="prefixForm.defense_modifier" :precision="2" :step="0.1" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="prefixDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="savePrefix">确定</el-button>
        </span>
      </template>
    </el-dialog>

    <!-- Rank Dialog -->
    <el-dialog v-model="rankDialogVisible" :title="editingRank ? '编辑等阶' : '新增等阶'" width="500px">
      <el-form :model="rankForm" label-width="120px">
        <el-form-item label="名称 (Name)">
          <el-input v-model="rankForm.name" />
        </el-form-item>
        <el-form-item label="生命系数">
          <el-input-number v-model="rankForm.hp_modifier" :precision="2" :step="0.1" />
        </el-form-item>
        <el-form-item label="攻击系数">
          <el-input-number v-model="rankForm.attack_modifier" :precision="2" :step="0.1" />
        </el-form-item>
        <el-form-item label="防御系数">
          <el-input-number v-model="rankForm.defense_modifier" :precision="2" :step="0.1" />
        </el-form-item>
        <el-form-item label="颜色 (Color)">
          <el-color-picker v-model="rankForm.color" />
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="rankDialogVisible = false">取消</el-button>
          <el-button type="primary" @click="saveRank">确定</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'

const props = defineProps({
  novelId: {
    type: Number,
    required: true
  }
})

const activeTab = ref('prefixes')

// Prefixes
const prefixes = ref([])
const loadingPrefixes = ref(false)
const prefixDialogVisible = ref(false)
const editingPrefix = ref(null)
const prefixForm = ref({
  name: '',
  hp_modifier: 1.0,
  attack_modifier: 1.0,
  defense_modifier: 1.0
})

// Ranks
const ranks = ref([])
const loadingRanks = ref(false)
const rankDialogVisible = ref(false)
const editingRank = ref(null)
const rankForm = ref({
  name: '',
  hp_modifier: 1.0,
  attack_modifier: 1.0,
  defense_modifier: 1.0,
  color: null
})

// Settings
const settings = ref({
  level_coefficient: 1.0
})
const loadingSettings = ref(false)

const handleTabClick = (tab) => {
  if (tab.paneName === 'prefixes') loadPrefixes()
  if (tab.paneName === 'ranks') loadRanks()
  if (tab.paneName === 'settings') loadSettings()
}

// Prefix Methods
const loadPrefixes = async () => {
  loadingPrefixes.value = true
  try {
    prefixes.value = await invoke('get_monster_prefixes', { novelId: props.novelId })
  } catch (error) {
    ElMessage.error('Failed to load prefixes: ' + error)
  } finally {
    loadingPrefixes.value = false
  }
}

const openPrefixDialog = (row = null) => {
  editingPrefix.value = row
  if (row) {
    prefixForm.value = { ...row }
  } else {
    prefixForm.value = { name: '', hp_modifier: 1.0, attack_modifier: 1.0, defense_modifier: 1.0 }
  }
  prefixDialogVisible.value = true
}

const savePrefix = async () => {
  try {
    if (editingPrefix.value) {
      await invoke('update_monster_prefix', {
        id: editingPrefix.value.id,
        name: prefixForm.value.name,
        hpModifier: prefixForm.value.hp_modifier,
        attackModifier: prefixForm.value.attack_modifier,
        defenseModifier: prefixForm.value.defense_modifier
      })
      ElMessage.success('Prefix updated')
    } else {
      await invoke('create_monster_prefix', {
        novelId: props.novelId,
        name: prefixForm.value.name,
        hpModifier: prefixForm.value.hp_modifier,
        attackModifier: prefixForm.value.attack_modifier,
        defenseModifier: prefixForm.value.defense_modifier
      })
      ElMessage.success('Prefix created')
    }
    prefixDialogVisible.value = false
    loadPrefixes()
  } catch (error) {
    ElMessage.error('Operation failed: ' + error)
  }
}

const deletePrefix = async (id) => {
  try {
    await ElMessageBox.confirm('Are you sure to delete this prefix?', 'Warning', {
      type: 'warning'
    })
    await invoke('delete_monster_prefix', { id })
    ElMessage.success('Prefix deleted')
    loadPrefixes()
  } catch (error) {
    if (error !== 'cancel') ElMessage.error('Failed to delete: ' + error)
  }
}

// Rank Methods
const loadRanks = async () => {
  loadingRanks.value = true
  try {
    ranks.value = await invoke('get_monster_ranks', { novelId: props.novelId })
  } catch (error) {
    ElMessage.error('Failed to load ranks: ' + error)
  } finally {
    loadingRanks.value = false
  }
}

const openRankDialog = (row = null) => {
  editingRank.value = row
  if (row) {
    rankForm.value = { ...row }
  } else {
    rankForm.value = { name: '', hp_modifier: 1.0, attack_modifier: 1.0, defense_modifier: 1.0, color: null }
  }
  rankDialogVisible.value = true
}

const saveRank = async () => {
  try {
    if (editingRank.value) {
      await invoke('update_monster_rank', {
        id: editingRank.value.id,
        name: rankForm.value.name,
        hpModifier: rankForm.value.hp_modifier,
        attackModifier: rankForm.value.attack_modifier,
        defenseModifier: rankForm.value.defense_modifier,
        color: rankForm.value.color
      })
      ElMessage.success('Rank updated')
    } else {
      await invoke('create_monster_rank', {
        novelId: props.novelId,
        name: rankForm.value.name,
        hpModifier: rankForm.value.hp_modifier,
        attackModifier: rankForm.value.attack_modifier,
        defenseModifier: rankForm.value.defense_modifier,
        color: rankForm.value.color
      })
      ElMessage.success('Rank created')
    }
    rankDialogVisible.value = false
    loadRanks()
  } catch (error) {
    ElMessage.error('Operation failed: ' + error)
  }
}

const deleteRank = async (id) => {
  try {
    await ElMessageBox.confirm('Are you sure to delete this rank?', 'Warning', {
      type: 'warning'
    })
    await invoke('delete_monster_rank', { id })
    ElMessage.success('Rank deleted')
    loadRanks()
  } catch (error) {
    if (error !== 'cancel') ElMessage.error('Failed to delete: ' + error)
  }
}

// Settings Methods
const loadSettings = async () => {
  loadingSettings.value = true
  try {
    const data = await invoke('get_game_settings', { novelId: props.novelId })
    settings.value = data
  } catch (error) {
    ElMessage.error('Failed to load settings: ' + error)
  } finally {
    loadingSettings.value = false
  }
}

const saveSettings = async () => {
  try {
    await invoke('update_game_settings', {
      novelId: props.novelId,
      levelCoefficient: settings.value.level_coefficient
    })
    ElMessage.success('Settings saved')
  } catch (error) {
    ElMessage.error('Failed to save settings: ' + error)
  }
}

onMounted(() => {
  loadPrefixes()
  // Pre-load others?
})
</script>

<style scoped>
.monster-config {
  padding: 20px;
}
.tab-content {
  padding: 20px 0;
}
.header {
  margin-bottom: 20px;
}
.form-content {
  max-width: 600px;
}
.help-text {
  font-size: 12px;
  color: #909399;
  line-height: 1.5;
  margin-top: 5px;
}
</style>
