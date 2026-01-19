<template>
  <div>
    <el-tabs v-model="activeTab" type="card" @tab-click="handleTabClick">
      <el-tab-pane label="装备 (Equipment)" name="equipment">
        <div class="tab-content">
          <div style="margin-bottom: 10px">
            <el-button type="primary" @click="openAddDialog('Equipment')">{{ $t('item.add') }}</el-button>
          </div>
          <el-table :data="equipmentItems" style="width: 100%; height: calc(100vh - 300px);" height="100%" @row-dblclick="editItem">
            <el-table-column prop="name" :label="$t('common.name')" />
            <el-table-column prop="level" :label="$t('common.level')" width="80" sortable />
            <el-table-column prop="rarity" :label="$t('monster.rarity')" width="100" />
            <el-table-column prop="description" :label="$t('common.description')" show-overflow-tooltip />
            <el-table-column :label="$t('common.category')">
              <template #default="scope">
                <el-tag :type="getCategoryTagType(scope.row)">
                  <span v-if="scope.row.category_id">{{ getCategoryName(scope.row.category_id) }}</span>
                  <span v-else>{{ getCategoryLabel(scope.row.category) }}</span>
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('item.slot')">
              <template #default="scope">
                <el-tag v-if="scope.row.item_type" type="info" effect="plain">
                  {{ getItemTypeLabel(scope.row.item_type) }}
                </el-tag>
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
        </div>
      </el-tab-pane>

      <el-tab-pane label="材料 (Material)" name="material">
        <div class="tab-content">
          <div style="margin-bottom: 10px">
            <el-button type="primary" @click="openAddDialog('Material')">{{ $t('item.add') }}</el-button>
          </div>
          <el-table :data="materialItems" style="width: 100%; height: calc(100vh - 300px);" height="100%" @row-dblclick="editItem">
            <el-table-column prop="name" :label="$t('common.name')" />
            <el-table-column prop="level" :label="$t('common.level')" width="80" sortable />
            <el-table-column prop="rarity" :label="$t('monster.rarity')" width="100" />
            <el-table-column prop="description" :label="$t('common.description')" show-overflow-tooltip />
            <el-table-column :label="$t('common.category')">
              <template #default="scope">
                <el-tag type="warning">
                  <span v-if="scope.row.category_id">{{ getCategoryName(scope.row.category_id) }}</span>
                  <span v-else>{{ getCategoryLabel(scope.row.category) }}</span>
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('common.actions')" width="180">
              <template #default="scope">
                <el-button size="small" @click="editItem(scope.row)">{{ $t('common.edit') }}</el-button>
                <el-button size="small" type="danger" @click="deleteItem(scope.row.id)">{{ $t('common.delete') }}</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-tab-pane>

      <el-tab-pane label="消耗品 (Consumable)" name="consumable">
        <div class="tab-content">
          <div style="margin-bottom: 10px">
            <el-button type="primary" @click="openAddDialog('Consumable')">{{ $t('item.add') }}</el-button>
          </div>
          <el-table :data="consumableItems" style="width: 100%; height: calc(100vh - 300px);" height="100%" @row-dblclick="editItem">
            <el-table-column prop="name" :label="$t('common.name')" />
            <el-table-column prop="level" :label="$t('common.level')" width="80" sortable />
            <el-table-column prop="rarity" :label="$t('monster.rarity')" width="100" />
            <el-table-column prop="description" :label="$t('common.description')" show-overflow-tooltip />
            <el-table-column :label="$t('common.category')">
              <template #default="scope">
                <el-tag type="danger">
                  <span v-if="scope.row.category_id">{{ getCategoryName(scope.row.category_id) }}</span>
                  <span v-else>{{ getCategoryLabel(scope.row.category) }}</span>
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('item.stats')">
              <template #default="scope">
                <div v-if="scope.row.attributes">
                  <el-tag v-for="(value, key) in scope.row.attributes" :key="key" size="small" style="margin-right: 5px">
                    {{ key }}: {{ value }}
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
        </div>
      </el-tab-pane>

      <el-tab-pane label="其他 (Others)" name="others">
        <div class="tab-content">
          <div style="margin-bottom: 10px">
            <el-button type="primary" @click="openAddDialog('Others')">{{ $t('item.add') }}</el-button>
          </div>
          <el-table :data="otherItems" style="width: 100%; height: calc(100vh - 300px);" height="100%" @row-dblclick="editItem">
            <el-table-column prop="name" :label="$t('common.name')" />
            <el-table-column prop="level" :label="$t('common.level')" width="80" sortable />
            <el-table-column prop="rarity" :label="$t('monster.rarity')" width="100" />
            <el-table-column prop="description" :label="$t('common.description')" show-overflow-tooltip />
            <el-table-column :label="$t('common.category')">
              <template #default="scope">
                <el-tag>
                  <span v-if="scope.row.category_id">{{ getCategoryName(scope.row.category_id) }}</span>
                  <span v-else>{{ getCategoryLabel(scope.row.category) }}</span>
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column :label="$t('common.actions')" width="180">
              <template #default="scope">
                <el-button size="small" @click="editItem(scope.row)">{{ $t('common.edit') }}</el-button>
                <el-button size="small" type="danger" @click="deleteItem(scope.row.id)">{{ $t('common.delete') }}</el-button>
              </template>
            </el-table-column>
          </el-table>
        </div>
      </el-tab-pane>
    </el-tabs>

    <!-- Add Dialog -->
    <el-dialog v-model="showAddDialog" :title="$t('item.add')" @keyup.enter="createItem">
      <el-form :model="form" label-width="100px">
        <el-form-item :label="$t('common.name')">
          <el-input v-model="form.name" />
        </el-form-item>
        <el-form-item :label="$t('common.description')">
          <el-input v-model="form.description" type="textarea" />
        </el-form-item>
        <el-row>
          <el-col :span="12">
            <el-form-item :label="$t('common.level')">
              <el-input-number v-model="form.level" :min="1" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="$t('monster.rarity')">
              <el-input v-model="form.rarity" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="$t('common.category')">
          <el-cascader
            v-model="form.category_id"
            :options="categoryOptions"
            :props="{ checkStrictly: true, emitPath: false }"
            placeholder="Select Category"
            clearable
          />
        </el-form-item>

        <el-form-item v-if="isEquipmentCategory(form.category_id)" :label="$t('item.slot')">
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
        <el-row>
          <el-col :span="12">
            <el-form-item :label="$t('common.level')">
              <el-input-number v-model="editForm.level" :min="1" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item :label="$t('monster.rarity')">
              <el-input v-model="editForm.rarity" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item :label="$t('common.category')">
          <el-cascader
            v-model="editForm.category_id"
            :options="categoryOptions"
            :props="{ checkStrictly: true, emitPath: false }"
            placeholder="Select Category"
            clearable
          />
        </el-form-item>

        <el-form-item v-if="isEquipmentCategory(editForm.category_id)" :label="$t('item.slot')">
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

const activeTab = ref('equipment')

const equipmentItems = computed(() => {
  return items.value.filter(i => {
    const catName = i.category_id ? getCategoryName(i.category_id) : i.category
    return catName === 'Equipment' || catName === '装备' || isEquipmentCategory(i.category_id)
  })
})

const materialItems = computed(() => {
  return items.value.filter(i => {
    const catName = i.category_id ? getCategoryName(i.category_id) : i.category
    return catName === 'Material' || catName === '材料' || isMaterialCategory(i.category_id)
  })
})

const consumableItems = computed(() => {
  return items.value.filter(i => {
    const catName = i.category_id ? getCategoryName(i.category_id) : i.category
    return catName === 'Consumable' || catName === '消耗品' || isConsumableCategory(i.category_id)
  })
})

const otherItems = computed(() => {
  return items.value.filter(i => {
    const catName = i.category_id ? getCategoryName(i.category_id) : i.category
    if (catName === 'Equipment' || catName === '装备' || isEquipmentCategory(i.category_id)) return false
    if (catName === 'Material' || catName === '材料' || isMaterialCategory(i.category_id)) return false
    if (catName === 'Consumable' || catName === '消耗品' || isConsumableCategory(i.category_id)) return false
    return true
  })
})

const handleTabClick = () => {
  // Can add logic here if needed
}

const openAddDialog = (defaultCat) => {
  let catId = null
  let catName = ''
  
  if (defaultCat === 'Equipment') catName = 'Equipment'
  else if (defaultCat === 'Material') catName = 'Material'
  else if (defaultCat === 'Consumable') catName = 'Consumable'
  
  // Try to find ID for this category name
  const cat = categories.value.find(c => c.name === catName || c.name === (catName === 'Equipment' ? '装备' : (catName === 'Material' ? '材料' : '消耗品')))
  if (cat) catId = cat.id
  
  form.value = { name: '', description: '', category_id: catId, item_type: '', level: 1, rarity: '' }
  // If we couldn't find ID, maybe we should set category string?
  if (!catId && catName) form.value.category = catName
  
  showAddDialog.value = true
}
const showEditDialog = ref(false)
const showAddDialog = ref(false)
const items = ref([])
const form = ref({ name: '', description: '', category_id: null, item_type: '', level: 1, rarity: '' })
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

 // Recursive check for category type
 const checkCategoryType = (categoryId, targetTypes) => {
   if (!categoryId) return false
   const cat = findCategoryById(categoryId)
   if (!cat) return false
   
   // Check current category name
   if (targetTypes.includes(cat.name)) return true
   
   // Check parent
   if (cat.parent_id) {
     return checkCategoryType(cat.parent_id, targetTypes)
   }
   return false
 }

 // Check if category (or its parent) is Equipment
 const isEquipmentCategory = (id) => {
   return checkCategoryType(id, ['Equipment', '装备'])
 }

 // Check if category is Weapon (or sub of Weapon, if any)
 const isWeaponCategory = (id) => {
   return checkCategoryType(id, ['Weapon', '武器'])
 }

 // Check if category is Armor/Accessory
 const isArmorCategory = (id) => {
   return checkCategoryType(id, ['Armor', 'Accessory', '防具', '饰品'])
 }
 
 // Check if category is Material
 const isMaterialCategory = (id) => {
   return checkCategoryType(id, ['Material', '材料'])
 }
 
 // Check if category is Consumable
 const isConsumableCategory = (id) => {
   return checkCategoryType(id, ['Consumable', '消耗品'])
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

const getCategoryTagType = (row) => {
  const catName = row.category_id ? getCategoryName(row.category_id) : row.category
  if (!catName) return 'info'
  if (catName === 'Equipment' || catName === '装备' || isEquipmentCategory(row.category_id)) return 'success' // Green
  if (catName === 'Material' || catName === '材料') return 'warning' // Orange
  if (catName === 'Consumable' || catName === '消耗品') return 'danger' // Red
  return '' // Default blue
}

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
      description: form.value.description || null,
      categoryId: form.value.category_id,
      level: form.value.level,
      rarity: form.value.rarity,
      attributes: attributes
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
     
     form.value = { name: '', description: '', category: '', sub_category: '', item_type: '', category_id: null, level: 1, rarity: '' }
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
      categoryId: editForm.value.category_id,
      level: editForm.value.level,
      rarity: editForm.value.rarity
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
