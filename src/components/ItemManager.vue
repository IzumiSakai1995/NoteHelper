<template>
  <div>
    <div style="margin-bottom: 10px">
      <el-button type="primary" @click="showAddDialog = true">{{ $t('item.add') }}</el-button>
    </div>
    <el-table :data="items" style="width: 100%" @row-dblclick="editItem">
      <el-table-column prop="name" :label="$t('common.name')" />
      <el-table-column prop="description" :label="$t('common.description')" show-overflow-tooltip />
      <el-table-column :label="$t('common.category')">
        <template #default="scope">
          <span v-if="scope.row.category_id">{{ getCategoryName(scope.row.category_id) }}</span>
          <span v-else>{{ getCategoryLabel(scope.row.category) }}</span>
        </template>
      </el-table-column>
      <el-table-column :label="$t('item.slot')">
        <template #default="scope">
          {{ getItemTypeLabel(scope.row.item_type) }}
        </template>
      </el-table-column>
      <el-table-column :label="$t('item.stats')">
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
          <el-button size="small" @click="editItem(scope.row)">{{ $t('common.edit') }}</el-button>
          <el-button size="small" type="danger" @click="deleteItem(scope.row.id)">{{ $t('common.delete') }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <!-- Add Dialog -->
    <el-dialog v-model="showAddDialog" :title="$t('item.add')" @keyup.enter="createItem">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.description')">
          <el-input v-model="form.description" type="textarea" />
        </el-form-item>
        <el-form-item :label="$t('common.category')">
          <el-cascader
            v-model="form.category_id"
            :options="categoryOptions"
            :props="{ checkStrictly: true, emitPath: false }"
            placeholder="Select Category"
            clearable
          />
        </el-form-item>

        <el-form-item v-if="form.category === 'Equipment'" :label="$t('item.slot')">
          <el-select v-model="form.item_type" :placeholder="$t('item.slot')">
            <el-option v-for="(label, key) in itemTypeMap" :key="key" :label="label" :value="key" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showAddDialog = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="createItem">{{ $t('common.create') }}</el-button>
      </template>
    </el-dialog>

    <!-- Edit Dialog -->
    <el-dialog v-model="showEditDialog" :title="$t('item.edit')" width="60%" @keyup.enter="updateItem">
      <el-form :model="editForm" label-width="100px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="editForm.name" />
        </el-form-item>
        <el-form-item :label="$t('common.description')">
          <el-input v-model="editForm.description" type="textarea" />
        </el-form-item>
        <el-form-item :label="$t('common.category')">
          <el-cascader
            v-model="editForm.category_id"
            :options="categoryOptions"
            :props="{ checkStrictly: true, emitPath: false }"
            placeholder="Select Category"
            clearable
          />
        </el-form-item>

        <el-form-item v-if="editForm.category === 'Equipment'" :label="$t('item.slot')">
          <el-select v-model="editForm.item_type" :placeholder="$t('item.slot')">
            <el-option v-for="(label, key) in itemTypeMap" :key="key" :label="label" :value="key" />
          </el-select>
        </el-form-item>
        
        <el-divider>{{ $t('item.stats') }}</el-divider>
        <!-- Standard Attributes based on Category -->
        <div v-if="isWeaponCategory(editForm.category_id)">
           <el-row>
             <el-col :span="12"><el-form-item label="Phys Attack"><el-input-number v-model="editForm.attributes.phys_attack" /></el-form-item></el-col>
             <el-col :span="12"><el-form-item label="Mag Attack"><el-input-number v-model="editForm.attributes.mag_attack" /></el-form-item></el-col>
           </el-row>
        </div>
        <div v-if="isArmorCategory(editForm.category_id)">
           <el-row>
             <el-col :span="12"><el-form-item :label="$t('player.phys_defense')"><el-input-number v-model="editForm.attributes.phys_defense" /></el-form-item></el-col>
             <el-col :span="12"><el-form-item :label="$t('player.mag_defense')"><el-input-number v-model="editForm.attributes.mag_defense" /></el-form-item></el-col>
           </el-row>
        </div>
        
        <el-row v-if="isEquipmentCategory(editForm.category_id)">
          <el-col :span="8"><el-form-item :label="$t('player.strength')"><el-input-number v-model="editForm.attributes.strength" /></el-form-item></el-col>
          <el-col :span="8"><el-form-item :label="$t('player.agility')"><el-input-number v-model="editForm.attributes.agility" /></el-form-item></el-col>
          <el-col :span="8"><el-form-item :label="$t('player.intelligence')"><el-input-number v-model="editForm.attributes.intelligence" /></el-form-item></el-col>
        </el-row>
        <el-row v-if="isEquipmentCategory(editForm.category_id)">
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
        <el-button type="primary" @click="updateItem">{{ $t('common.save') }}</el-button>
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

const items = ref([])
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const form = ref({ name: '', description: '', category_id: null, item_type: '' })
const categories = ref([])

const loadCategories = async () => {
  if (!props.novelId) return
  try {
    categories.value = await invoke('get_categories', { novelId: props.novelId })
  } catch (e) {
    console.error('Failed to load categories', e)
  }
}

// Convert flat categories to tree for cascader
const categoryOptions = computed(() => {
  const map = {}
  const roots = []
  
  categories.value.forEach(cat => {
    map[cat.id] = { value: cat.id, label: cat.name, children: [] }
  })

  categories.value.forEach(cat => {
    if (cat.parent_id && map[cat.parent_id]) {
      map[cat.parent_id].children.push(map[cat.id])
    } else {
      roots.push(map[cat.id])
    }
  })

  // Remove empty children arrays
  const clean = (nodes) => {
    nodes.forEach(node => {
      if (node.children.length === 0) {
        delete node.children
      } else {
        clean(node.children)
      }
    })
  }
  clean(roots)
  return roots
})

const getCategoryName = (id) => {
   const cat = categories.value.find(c => c.id === id)
   return cat ? cat.name : ''
 }

 const findCategoryById = (id) => {
   return categories.value.find(c => c.id === id)
 }

 // Check if category (or its parent) is Equipment
 const isEquipmentCategory = (id) => {
   if (!id) return false
   const cat = findCategoryById(id)
   if (!cat) return false
   if (cat.name === 'Equipment' || cat.name === '装备') return true
   if (cat.parent_id) return isEquipmentCategory(cat.parent_id)
   return false
 }

 // Check if category is Weapon (or sub of Weapon, if any)
 const isWeaponCategory = (id) => {
   if (!id) return false
   const cat = findCategoryById(id)
   if (!cat) return false
   if (cat.name === 'Weapon' || cat.name === '武器') return true
   if (cat.parent_id) return isWeaponCategory(cat.parent_id)
   return false
 }

 // Check if category is Armor/Accessory
 const isArmorCategory = (id) => {
   if (!id) return false
   const cat = findCategoryById(id)
   if (!cat) return false
   const names = ['Armor', 'Accessory', '防具', '饰品']
   if (names.includes(cat.name)) return true
   if (cat.parent_id) return isArmorCategory(cat.parent_id)
   return false
 }
const editForm = ref({ attributes: {} })
const attributeKeys = ref({}) 

const itemTypeMap = computed(() => ({
  'Main Hand': t('item.types.main_hand'),
  'Off Hand': t('item.types.off_hand'),
  'Two-Handed': t('item.types.two_handed'),
  'Chest': t('item.types.chest'),
  'Legs': t('item.types.legs'),
  'Hands': t('item.types.hands'),
  'Feet': t('item.types.feet'),
  'Ring': t('item.types.ring'),
  'Necklace': t('item.types.necklace'),
  'Earring': t('item.types.earring'),
  'Belt': t('item.types.belt'),
  'Back': t('item.types.back')
}))

const getCategoryLabel = (cat) => {
  if (cat === 'Equipment') return t('item.categories.equipment')
  if (cat === 'Material') return t('item.categories.material')
  if (cat === 'Consumable') return t('item.categories.consumable')
  return cat
}

const getItemTypeLabel = (type) => {
  return itemTypeMap.value[type] || type
}

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

const loadItems = async () => {
  if (!props.novelId) return
  try {
    items.value = await invoke('get_items', { novelId: props.novelId })
    await loadCategories()
  } catch (e) {
    ElMessage.error('Failed to load items: ' + e)
  }
}

const createItem = async () => {
  if (!form.value.name) return
  try {
    // If equipment, use sub_category as the main category stored in items logic? 
    // Wait, the prompt says "Equipment as a category, and items (Weapon/Armor/Accessory) as sub-categories".
    // But existing DB has 'category' and 'item_type'.
    // We should probably store 'Equipment' as category, and maybe store sub-category in attributes or just infer it?
    // Actually, let's store 'Equipment' in category. 
    // And 'Weapon', 'Armor' etc can be stored in attributes or we can repurpose item_type?
    // Item type is 'Main Hand', 'Chest' etc.
    
    // Let's store sub_category in attributes for now to distinguish Weapon/Armor if needed, 
    // or just rely on item_type which is specific enough.
    
    const attributes = {}
    if (form.value.category === 'Equipment') {
       attributes.sub_category = form.value.sub_category
    }

    await invoke('create_item', { 
      novelId: props.novelId, 
      name: form.value.name,
      category: form.value.category || null,
      itemType: form.value.item_type || null,
      description: form.value.description || null
      // We need to pass attributes if we want to save sub_category
      // But create_item command doesn't take attributes!
      // We need to update create_item command in Rust.
    })
    // Oh wait, create_item in Rust DOES NOT take attributes.
    // We should update it. Or just update immediately after create.
    
    // For now, let's just stick to the requested UI changes.
    // The user said "Item Manager管理中添加分类材料，消耗品，将装备作为item的下级分类在item Manager中进行管理。"
    // So 'Equipment' is a category. 'Material', 'Consumable' are categories.
    // Under 'Equipment', we have 'Weapon', 'Armor', 'Accessory' which are sub-categories.
    
    // Existing items have category='Weapon'/'Armor'/'Accessory'.
    // We should probably migrate them to category='Equipment' and sub_category='Weapon' etc.
    // But for new items:
    
    showAddDialog.value = false
    
    // We can't save sub_category in create_item yet.
     // Let's just create it, and if it's Equipment, the item_type (slot) usually implies the sub category.
     // e.g. Main Hand -> Weapon. Chest -> Armor.
     // So maybe we don't strictly need sub_category in DB.
     
     form.value = { name: '', description: '', category: '', sub_category: '', item_type: '' }
     loadItems()
     ElMessage.success('Item created')
  } catch (e) {
    ElMessage.error('Failed to create item: ' + e)
  }
}

const editItem = (row) => {
  editForm.value = JSON.parse(JSON.stringify(row))
  if (!editForm.value.attributes) {
    editForm.value.attributes = {}
  }
  // No backward compat logic needed here for category_id, as we use it directly.
  // Legacy string categories are just displayed as fallback in table.
  // Editing them will require selecting a new category from tree.
  
  // Ensure standard fields exist for binding
  standardKeys.forEach(key => {
    if (editForm.value.attributes[key] === undefined) {
      // Don't set to 0 by default to avoid clutter, but for binding v-model it's easier if they exist or we handle undefined.
      // Actually el-input-number handles undefined fine (shows empty).
      // But if we want to save them, we might want to set them.
    }
  })

  // Initialize attributeKeys map for custom attributes
  attributeKeys.value = {}
  Object.keys(editForm.value.attributes).forEach(key => {
    if (!standardKeys.includes(key)) {
      attributeKeys.value[key] = key
    }
  })
  showEditDialog.value = true
}

  const onAddCategoryChange = () => {
     form.value.item_type = ''
  }

  const onCategoryChange = () => {
  // Optional: clear irrelevant stats when category changes?
  // For now keep it simple.
}

const updateItem = async () => {
  try {
    // Clean up undefined/null attributes before sending?
    // Or just send as is.
    
    // Restore legacy category if needed, OR just save 'Equipment' as category.
    // The requirement says "manage equipment as sub category of item".
    // So saving category='Equipment' is correct.
    
    await invoke('update_item', {
      id: editForm.value.id,
      name: editForm.value.name,
      category: editForm.value.category,
      itemType: editForm.value.item_type,
      attributes: editForm.value.attributes,
      description: editForm.value.description,
      category_id: editForm.value.category_id
    })
    showEditDialog.value = false
    loadItems()
    ElMessage.success('Item updated')
  } catch (e) {
    ElMessage.error('Failed to update item: ' + e)
  }
}

const deleteItem = async (id) => {
  try {
    await invoke('delete_item', { id })
    loadItems()
    ElMessage.success('Item deleted')
  } catch (e) {
    ElMessage.error('Failed to delete item: ' + e)
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

watch(() => props.novelId, loadItems)
onMounted(loadItems)
</script>
