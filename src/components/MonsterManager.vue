<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('monster.add') }}</el-button>
    </div>
    <el-table 
      :data="monsters" 
      style="width: 100%; height: calc(100vh - 200px);" 
      @row-dblclick="editMonster"
      height="100%"
      :default-sort="{ prop: 'level', order: 'ascending' }"
    >
      <el-table-column prop="name" :label="$t('common.name')">
        <template #default="scope">
          <span :style="{ color: getRankColor(scope.row.rank_id) }">{{ scope.row.name }}</span>
        </template>
      </el-table-column>
      <el-table-column prop="level" :label="$t('common.level')" sortable />
      <el-table-column prop="hp" :label="$t('player.hp')" />
      <el-table-column prop="attack" :label="$t('player.attack')" />
      <el-table-column :label="$t('common.actions')">
        <template #default="scope">
          <el-button size="small" @click="editMonster(scope.row)">{{ $t('common.edit') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showAddDialog" :title="$t('monster.add')" width="70%" @keyup.enter="createMonster">
      <el-form :model="form" label-width="120px">
        <el-row>
          <el-col :span="12"><el-form-item :label="$t('common.name')"><el-input v-model="form.name" /></el-form-item></el-col>
          <el-col :span="12"><el-form-item :label="$t('common.level')"><el-input-number v-model="form.level" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="24"><el-form-item :label="$t('monster.race')"><el-input v-model="form.race" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="前缀 (Prefix)">
              <el-select v-model="form.prefix_id" clearable placeholder="Select Prefix">
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
              <el-select v-model="form.rank_id" clearable placeholder="Select Rank">
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
              <el-input-number v-model="form.base_hp" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="基础攻击 (Base Atk)">
              <el-input-number v-model="form.base_attack" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="基础防御 (Base Def)">
              <el-input-number v-model="form.base_defense" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="$t('monster.damage_reduction') + ' (%)'">
              <el-input-number v-model="form.damage_reduction" :min="0" :max="100" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="固定减伤 (Fixed Red.)">
              <el-input-number v-model="form.fixed_damage_reduction" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="$t('monster.description')">
          <el-input v-model="form.description" type="textarea" />
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
          <el-col :span="24"><el-form-item :label="$t('monster.race')"><el-input v-model="editForm.race" /></el-form-item></el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="前缀 (Prefix)">
              <el-select v-model="editForm.prefix_id" clearable placeholder="Select Prefix">
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
              <el-select v-model="editForm.rank_id" clearable placeholder="Select Rank">
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
              <el-input-number v-model="editForm.base_hp" />
              <div style="font-size: 12px; color: #999;">Final: {{ editForm.hp }}</div>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="基础攻击 (Base Atk)">
              <el-input-number v-model="editForm.base_attack" />
              <div style="font-size: 12px; color: #999;">Final: {{ editForm.attack }}</div>
            </el-form-item>
          </el-col>
        </el-row>
        <el-row>
          <el-col :span="12">
            <el-form-item label="基础防御 (Base Def)">
              <el-input-number v-model="editForm.base_defense" />
              <div style="font-size: 12px; color: #999;">Final: {{ editForm.defense }}</div>
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="$t('monster.damage_reduction') + ' (%)'">
              <el-input-number v-model="editForm.damage_reduction" :min="0" :max="100" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="固定减伤 (Fixed Red.)">
              <el-input-number v-model="editForm.fixed_damage_reduction" />
            </el-form-item>
          </el-col>
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
const form = ref({ 
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
const editForm = ref({})
const prefixes = ref([])
const ranks = ref([])

const loadMetadata = async () => {
  if (!props.novelId) return
  try {
    prefixes.value = await invoke('get_monster_prefixes', { novelId: props.novelId })
    ranks.value = await invoke('get_monster_ranks', { novelId: props.novelId })
  } catch (e) {
    console.error('Failed to load metadata', e)
  }
}

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
    await invoke('create_monster', { 
      novelId: props.novelId, 
      name: form.value.name,
      level: form.value.level,
      race: form.value.race,
      prefixId: form.value.prefix_id,
      rankId: form.value.rank_id,
      baseHp: form.value.base_hp,
      baseAttack: form.value.base_attack,
      baseDefense: form.value.base_defense,
      damageReduction: form.value.damage_reduction,
      fixedDamageReduction: form.value.fixed_damage_reduction,
      description: form.value.description
    })
    showAddDialog.value = false
    // Reset form
    form.value = { 
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
      baseHp: editForm.value.base_hp,
      baseAttack: editForm.value.base_attack,
      baseDefense: editForm.value.base_defense,
      damageReduction: editForm.value.damage_reduction,
      fixedDamageReduction: editForm.value.fixed_damage_reduction,
      drops: editForm.value.drops,
      description: editForm.value.description,
      prefixId: editForm.value.prefix_id,
      rankId: editForm.value.rank_id
    })
    showEditDialog.value = false
    loadMonsters()
    ElMessage.success('Monster updated')
  } catch (e) {
    ElMessage.error('Failed to update monster: ' + e)
  }
}

const getRankColor = (rankId) => {
  if (!rankId) return '#606266'
  const rank = ranks.value.find(r => r.id === rankId)
  return rank ? (rank.color || '#606266') : '#606266'
}

watch(() => props.novelId, () => {
  loadMonsters()
  loadMetadata()
})
onMounted(() => {
  loadMonsters()
  loadMetadata()
})
</script>
