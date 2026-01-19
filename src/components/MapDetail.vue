<template>
  <div class="map-detail">
    <div class="header">
      <el-button @click="$emit('back')" icon="ArrowLeft">{{ $t('common.back') }}</el-button>
      <h2 style="margin-left: 20px; display: inline-block;">{{ mapData.name }}</h2>
    </div>

    <el-tabs v-model="activeTab">
      <el-tab-pane :label="$t('tabs.monsters')" name="monsters">
        <div style="margin-bottom: 10px">
          <el-button type="primary" @click="$emit('bind-monster', mapData)">{{ $t('map.bind_monsters') }}</el-button>
          <el-button type="success" @click="$emit('add-monster', mapData)">{{ $t('monster.add') }}</el-button>
        </div>
        <el-table :data="mapMonsters" style="width: 100%">
          <el-table-column prop="name" :label="$t('common.name')">
             <template #default="scope">
                <div @dblclick.stop="startEdit(scope.row, 'name', 'monster')" style="cursor: text; min-height: 20px;">
                  <el-input 
                    v-if="isEditing(scope.row.id, 'name', 'monster')" 
                    v-model="editingValue" 
                    @blur="saveMonster(scope.row, 'name')" 
                    @keyup.enter="saveMonster(scope.row, 'name')"
                    v-focus
                    size="small"
                  />
                  <span v-else :style="{ color: getRankColor(scope.row.rank_id) }">{{ scope.row.name }}</span>
                </div>
             </template>
          </el-table-column>
          <el-table-column prop="level" :label="$t('common.level')" width="80">
            <template #default="scope">
               <div @dblclick.stop="startEdit(scope.row, 'level', 'monster')" style="cursor: text; min-height: 20px;">
                  <el-input-number 
                    v-if="isEditing(scope.row.id, 'level', 'monster')" 
                    v-model="editingValue" 
                    @blur="saveMonster(scope.row, 'level')" 
                    @change="saveMonster(scope.row, 'level')"
                    v-focus
                    size="small"
                    :controls="false"
                    style="width: 100%"
                  />
                  <span v-else>{{ scope.row.level }}</span>
               </div>
            </template>
          </el-table-column>
          <el-table-column :label="$t('monster.prefix')" width="120">
            <template #default="scope">
              {{ getPrefixName(scope.row.prefix_id) }}
            </template>
          </el-table-column>
          <el-table-column :label="$t('monster.rank')" width="120">
            <template #default="scope">
              {{ getRankName(scope.row.rank_id) }}
            </template>
          </el-table-column>
          <el-table-column :label="$t('common.stats')">
            <template #default="scope">
              HP: {{ scope.row.hp }} / Atk: {{ scope.row.attack }} / Def: {{ scope.row.defense }}
            </template>
          </el-table-column>
          <el-table-column :label="$t('common.actions')" width="150">
            <template #default="scope">
              <el-button size="small" @click="$emit('edit-monster', scope.row)">{{ $t('common.edit') }}</el-button>
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>

      <el-tab-pane :label="$t('map.sub_maps')" name="submaps">
        <el-table 
          :data="subMaps" 
          style="width: 100%" 
          @row-dblclick="(row) => $emit('navigate', row.id)"
        >
          <el-table-column prop="order_num" :label="$t('common.order')" width="80">
             <template #default="scope">
                <div @dblclick.stop="startEdit(scope.row, 'order_num', 'map')" style="cursor: text; min-height: 20px;">
                   <el-input-number 
                     v-if="isEditing(scope.row.id, 'order_num', 'map')" 
                     v-model="editingValue" 
                     @blur="saveSubMap(scope.row, 'order_num')" 
                     @change="saveSubMap(scope.row, 'order_num')"
                     v-focus
                     size="small"
                     :controls="false"
                     style="width: 100%"
                   />
                   <span v-else>{{ scope.row.order_num }}</span>
                </div>
             </template>
          </el-table-column>
          <el-table-column prop="name" :label="$t('common.name')">
             <template #default="scope">
                <div @dblclick.stop="startEdit(scope.row, 'name', 'map')" style="cursor: text; min-height: 20px;">
                  <el-input 
                    v-if="isEditing(scope.row.id, 'name', 'map')" 
                    v-model="editingValue" 
                    @blur="saveSubMap(scope.row, 'name')" 
                    @keyup.enter="saveSubMap(scope.row, 'name')"
                    v-focus
                    size="small"
                  />
                  <span v-else>{{ scope.row.name }}</span>
                </div>
             </template>
          </el-table-column>
          <el-table-column prop="description" :label="$t('monster.description')" show-overflow-tooltip>
             <template #default="scope">
                <div @dblclick.stop="startEdit(scope.row, 'description', 'map')" style="cursor: text; min-height: 20px;">
                  <el-input 
                    v-if="isEditing(scope.row.id, 'description', 'map')" 
                    v-model="editingValue" 
                    @blur="saveSubMap(scope.row, 'description')" 
                    @keyup.enter="saveSubMap(scope.row, 'description')"
                    v-focus
                    size="small"
                  />
                  <span v-else>{{ scope.row.description }}</span>
                </div>
             </template>
          </el-table-column>
          <el-table-column :label="$t('tabs.monsters')">
            <template #default="scope">
              {{ (scope.row.monsters || []).length }} monsters
            </template>
          </el-table-column>
        </el-table>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  mapData: Object,
  subMaps: Array,
  mapMonsters: Array,
  prefixes: Array,
  ranks: Array
})

const activeTab = ref('monsters')

const getPrefixName = (id) => {
  if (!id) return ''
  const p = props.prefixes.find(x => x.id === id)
  return p ? p.name : ''
}

const getRankName = (id) => {
  if (!id) return ''
  const r = props.ranks.find(x => x.id === id)
  return r ? r.name : ''
}

const getRankColor = (rankId) => {
  if (!rankId) return '#606266'
  const rank = props.ranks.find(r => r.id === rankId)
  return rank ? (rank.color || '#606266') : '#606266'
}

// Inline Editing Logic
const editingCell = ref(null) // { id: number, field: string, type: 'map'|'monster' }
const editingValue = ref('')

const isEditing = (id, field, type) => {
  return editingCell.value && 
         editingCell.value.id === id && 
         editingCell.value.field === field && 
         editingCell.value.type === type
}

const startEdit = (row, field, type) => {
  editingCell.value = { id: row.id, field, type }
  editingValue.value = row[field]
}

const emit = defineEmits(['back', 'navigate', 'edit-monster', 'bind-monster', 'add-monster', 'update-monster-inline', 'update-submap'])

const saveMonster = (row, field) => {
  if (!editingCell.value) return
  if (editingValue.value !== row[field]) {
    emit('update-monster-inline', { ...row, [field]: editingValue.value })
  }
  editingCell.value = null
}

const saveSubMap = (row, field) => {
  if (!editingCell.value) return
  if (editingValue.value !== row[field]) {
    emit('update-submap', { ...row, [field]: editingValue.value })
  }
  editingCell.value = null
}

const vFocus = {
  mounted: (el) => {
    const input = el.querySelector('input') || el.querySelector('textarea')
    if (input) input.focus()
  }
}
</script>

<style scoped>
.header {
  margin-bottom: 20px;
  display: flex;
  align-items: center;
}
</style>
