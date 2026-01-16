<template>
  <div>
    <div style="margin-bottom: 10px; display: flex; gap: 10px">
      <el-button type="primary" @click="openAddDialog(null)">{{ $t('category.add_root') }}</el-button>
      <el-button @click="expandAll">{{ $t('common.expand_all') }}</el-button>
      <el-button @click="collapseAll">{{ $t('common.collapse_all') }}</el-button>
    </div>

    <el-table
      :data="categoryTree"
      style="width: 100%; margin-bottom: 20px;"
      row-key="id"
      border
      default-expand-all
      ref="categoryTable"
    >
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="description" :label="$t('common.description')" />
      <el-table-column :label="$t('common.actions')" width="250">
        <template #default="scope">
          <el-button size="small" type="primary" link @click="openAddDialog(scope.row)">
            {{ $t('category.add_sub') }}
          </el-button>
          <el-button size="small" link @click="openEditDialog(scope.row)">
            {{ $t('common.edit') }}
          </el-button>
          <el-button size="small" type="danger" link @click="deleteCategory(scope.row.id)">
            {{ $t('common.delete') }}
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- Dialog -->
    <el-dialog v-model="showDialog" :title="dialogTitle" width="500px">
      <el-form :model="form" label-width="100px">
        <el-form-item v-if="form.parentName" :label="$t('category.parent')">
          <el-input v-model="form.parentName" disabled />
        </el-form-item>
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.description')">
          <el-input v-model="form.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="submitForm">{{ $t('common.save') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const props = defineProps({
  novelId: Number
})

const categories = ref([])
const showDialog = ref(false)
const isEdit = ref(false)
const form = ref({
  id: null,
  novel_id: null,
  parent_id: null,
  parentName: '',
  name: '',
  description: ''
})

const categoryTable = ref(null)

const categoryTree = computed(() => {
  const map = {}
  const roots = []
  
  // First pass: create nodes
  categories.value.forEach(cat => {
    map[cat.id] = { ...cat, children: [] }
  })

  // Second pass: link parents
  categories.value.forEach(cat => {
    if (cat.parent_id && map[cat.parent_id]) {
      map[cat.parent_id].children.push(map[cat.id])
    } else {
      roots.push(map[cat.id])
    }
  })

  return roots
})

const dialogTitle = computed(() => {
  return isEdit.value ? t('category.edit') : t('category.add')
})

const loadCategories = async () => {
  if (!props.novelId) return
  try {
    categories.value = await invoke('get_categories', { novelId: props.novelId })
  } catch (e) {
    ElMessage.error('Failed to load categories: ' + e)
  }
}

const openAddDialog = (parent) => {
  isEdit.value = false
  form.value = {
    id: null,
    novel_id: props.novelId,
    parent_id: parent ? parent.id : null,
    parentName: parent ? parent.name : '',
    name: '',
    description: ''
  }
  showDialog.value = true
}

const openEditDialog = (row) => {
  isEdit.value = true
  const parent = categories.value.find(c => c.id === row.parent_id)
  form.value = {
    id: row.id,
    novel_id: row.novel_id,
    parent_id: row.parent_id,
    parentName: parent ? parent.name : '',
    name: row.name,
    description: row.description
  }
  showDialog.value = true
}

const submitForm = async () => {
  if (!form.value.name) {
    ElMessage.warning(t('common.name_required'))
    return
  }

  try {
    if (isEdit.value) {
      await invoke('update_category', {
        id: form.value.id,
        name: form.value.name,
        description: form.value.description || null
      })
      ElMessage.success(t('common.updated'))
    } else {
      await invoke('create_category', {
        novelId: props.novelId,
        parentId: form.value.parent_id,
        name: form.value.name,
        description: form.value.description || null
      })
      ElMessage.success(t('common.created'))
    }
    showDialog.value = false
    loadCategories()
  } catch (e) {
    ElMessage.error('Operation failed: ' + e)
  }
}

const deleteCategory = async (id) => {
  try {
    await ElMessageBox.confirm(t('common.confirm_delete'), t('common.warning'), {
      confirmButtonText: t('common.confirm'),
      cancelButtonText: t('common.cancel'),
      type: 'warning'
    })
    
    await invoke('delete_category', { id })
    ElMessage.success(t('common.deleted'))
    loadCategories()
  } catch (e) {
    if (e !== 'cancel') {
      ElMessage.error('Failed to delete: ' + e)
    }
  }
}

const toggleExpansion = (nodes, expanded) => {
  nodes.forEach(node => {
    if (node.children && node.children.length > 0) {
      if (categoryTable.value) {
        categoryTable.value.toggleRowExpansion(node, expanded)
      }
      toggleExpansion(node.children, expanded)
    }
  })
}

const expandAll = () => {
  toggleExpansion(categoryTree.value, true)
}

const collapseAll = () => {
  toggleExpansion(categoryTree.value, false)
}

watch(() => props.novelId, loadCategories)
onMounted(loadCategories)
</script>
