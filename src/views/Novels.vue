<template>
  <div class="container">
    <div class="header">
      <h1>{{ $t('novel.title') }}</h1>
      <div style="display: flex; gap: 10px; align-items: center;">
        <LanguageSwitcher />
        <el-button type="primary" @click="showAddDialog = true">{{ $t('novel.add') }}</el-button>
      </div>
    </div>

    <el-table :data="novels" style="width: 100%" @row-click="goToDetail" highlight-current-row cursor="pointer">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="description" :label="$t('common.description')" />
      <el-table-column :label="$t('common.actions')" width="120">
        <template #default="scope">
          <el-button size="small" type="danger" @click.stop="deleteNovel(scope.row.id)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-dialog v-model="showAddDialog" :title="$t('novel.add')">
      <el-form :model="form">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.description')">
          <el-input v-model="form.description" type="textarea" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createNovel">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import LanguageSwitcher from '../components/LanguageSwitcher.vue'

const router = useRouter()
const novels = ref([])
const showAddDialog = ref(false)
const form = ref({ name: '', description: '' })

const loadNovels = async () => {
  try {
    novels.value = await invoke('get_novels')
  } catch (e) {
    console.error(e)
    ElMessage.error('Failed to load novels: ' + e)
  }
}

const createNovel = async () => {
  if (!form.value.name) return
  try {
    await invoke('create_novel', { name: form.value.name, description: form.value.description })
    showAddDialog.value = false
    form.value = { name: '', description: '' }
    loadNovels()
    ElMessage.success('Novel created')
  } catch (e) {
    console.error(e)
    ElMessage.error('Failed to create novel: ' + e)
  }
}

const deleteNovel = async (id) => {
  try {
    await invoke('delete_novel', { id })
    loadNovels()
    ElMessage.success('Novel deleted')
  } catch (e) {
    console.error(e)
    ElMessage.error('Failed to delete novel: ' + e)
  }
}

const goToDetail = (row) => {
  router.push({ name: 'NovelDetail', params: { id: row.id } })
}

onMounted(loadNovels)
</script>

<style scoped>
.container {
  padding: 20px;
}
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}
</style>
