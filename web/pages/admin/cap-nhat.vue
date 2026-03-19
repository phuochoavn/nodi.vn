<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-slate-800 dark:text-white">Quản lý cập nhật</h1>
        <p class="mt-1 text-sm text-slate-500 dark:text-slate-400">Upload file cài đặt và quản lý phiên bản</p>
      </div>
      <button @click="loadData" :disabled="loading"
              class="flex items-center gap-2 px-4 py-2 rounded-lg bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 transition-colors text-sm font-medium text-slate-600 dark:text-slate-300">
        <RefreshCw :size="16" :class="{ 'animate-spin': loading }" />
        Làm mới
      </button>
    </div>

    <!-- Current Version Card -->
    <div class="bg-gradient-to-br from-primary/10 via-primary/5 to-transparent border border-primary/20 rounded-2xl p-6">
      <div class="flex flex-col md:flex-row md:items-center gap-6">
        <div class="w-16 h-16 rounded-2xl bg-primary/20 flex items-center justify-center flex-shrink-0">
          <Package :size="32" class="text-primary" />
        </div>
        <div class="flex-1">
          <div class="flex items-center gap-3 mb-1">
            <h2 class="text-xl font-bold text-slate-800 dark:text-white">Nodi POS</h2>
            <span class="px-2.5 py-0.5 rounded-full text-xs font-bold bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400">
              v{{ config.latest_version || '—' }}
            </span>
          </div>
          <p class="text-sm text-slate-500 dark:text-slate-400">
            <span v-if="config.file_size">{{ config.file_size }} · </span>
            <span v-if="config.updated_at">Cập nhật: {{ formatDate(config.updated_at) }}</span>
            <span v-else>Chưa có thông tin</span>
          </p>
          <p v-if="config.download_url" class="text-xs text-slate-400 dark:text-slate-500 mt-1 font-mono truncate">
            {{ config.download_url }}
          </p>
        </div>
        <a v-if="config.download_url" :href="config.download_url" target="_blank"
           class="flex items-center gap-2 px-4 py-2 rounded-lg bg-primary text-white font-medium text-sm hover:bg-primary/90 transition-colors flex-shrink-0">
          <Download :size="16" />
          Tải xuống
        </a>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Upload Zone -->
      <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 overflow-hidden">
        <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700">
          <h3 class="font-bold text-slate-800 dark:text-white flex items-center gap-2">
            <Upload :size="18" class="text-primary" />
            Upload bản cài đặt mới
          </h3>
        </div>
        <div class="p-6">
          <!-- Drop Zone -->
          <div @dragover.prevent="dragOver = true" @dragleave="dragOver = false" @drop.prevent="handleDrop"
               @click="$refs.fileInput.click()"
               class="border-2 border-dashed rounded-xl p-8 text-center cursor-pointer transition-all duration-200"
               :class="dragOver ? 'border-primary bg-primary/5' : 'border-slate-200 dark:border-slate-600 hover:border-primary/50 hover:bg-slate-50 dark:hover:bg-slate-700/50'">
            <input ref="fileInput" type="file" accept=".exe,.msi,.zip" class="hidden" @change="handleFileSelect" />
            <div v-if="!selectedFile" class="space-y-2">
              <CloudUpload :size="40" class="mx-auto text-slate-400" />
              <p class="text-sm font-medium text-slate-600 dark:text-slate-300">Kéo thả file hoặc click để chọn</p>
              <p class="text-xs text-slate-400">.exe, .msi, .zip — Tối đa 200 MB</p>
            </div>
            <div v-else class="space-y-2">
              <FileCheck :size="40" class="mx-auto text-green-500" />
              <p class="text-sm font-medium text-slate-800 dark:text-white">{{ selectedFile.name }}</p>
              <p class="text-xs text-slate-500">{{ formatFileSize(selectedFile.size) }}</p>
            </div>
          </div>

          <!-- Version + Notes -->
          <div class="mt-4 space-y-3">
            <div>
              <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">Phiên bản</label>
              <input v-model="uploadVersion" type="text" placeholder="1.1.0"
                     class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-sm text-slate-800 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
            </div>
            <div>
              <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">Ghi chú phiên bản</label>
              <textarea v-model="uploadNotes" rows="3" placeholder="- Sửa lỗi X&#10;- Thêm tính năng Y"
                        class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-sm text-slate-800 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none resize-none"></textarea>
            </div>
          </div>

          <!-- Upload Progress -->
          <div v-if="uploading" class="mt-4">
            <div class="flex items-center justify-between text-xs text-slate-500 mb-1">
              <span>Đang upload...</span>
              <span>{{ uploadProgress }}%</span>
            </div>
            <div class="w-full h-2 bg-slate-100 dark:bg-slate-700 rounded-full overflow-hidden">
              <div class="h-full bg-primary rounded-full transition-all duration-300" :style="{ width: uploadProgress + '%' }"></div>
            </div>
          </div>

          <!-- Upload Button -->
          <button @click="handleUpload" :disabled="!selectedFile || uploading"
                  class="mt-4 w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl font-bold text-sm transition-all duration-200"
                  :class="selectedFile && !uploading
                    ? 'bg-primary text-white hover:bg-primary/90 shadow-lg shadow-primary/25'
                    : 'bg-slate-100 dark:bg-slate-700 text-slate-400 cursor-not-allowed'">
            <Rocket :size="18" />
            {{ uploading ? 'Đang upload...' : 'Publish bản cập nhật' }}
          </button>

          <!-- Upload result -->
          <div v-if="uploadResult" class="mt-3 p-3 rounded-lg text-sm"
               :class="uploadResult.success ? 'bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400' : 'bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400'">
            {{ uploadResult.message }}
          </div>
        </div>
      </div>

      <!-- Edit Config -->
      <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 overflow-hidden">
        <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700">
          <h3 class="font-bold text-slate-800 dark:text-white flex items-center gap-2">
            <Settings :size="18" class="text-primary" />
            Chỉnh sửa thông tin
          </h3>
        </div>
        <div class="p-6 space-y-3">
          <div>
            <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">Phiên bản hiện tại</label>
            <input v-model="editVersion" type="text"
                   class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-sm text-slate-800 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
          </div>
          <div>
            <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">URL tải xuống</label>
            <input v-model="editUrl" type="text"
                   class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-xs text-slate-800 dark:text-white font-mono focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
          </div>
          <div>
            <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">Ghi chú phiên bản</label>
            <textarea v-model="editNotes" rows="3"
                      class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-sm text-slate-800 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none resize-none"></textarea>
          </div>
          <div>
            <label class="block text-xs font-semibold text-slate-500 dark:text-slate-400 mb-1">Kích thước file</label>
            <input v-model="editSize" type="text" placeholder="94.2 MB"
                   class="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700 text-sm text-slate-800 dark:text-white focus:ring-2 focus:ring-primary/50 focus:border-primary outline-none" />
          </div>
          <button @click="handleEditConfig" :disabled="saving"
                  class="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl font-bold text-sm bg-slate-800 dark:bg-white text-white dark:text-slate-800 hover:bg-slate-700 dark:hover:bg-slate-100 transition-all duration-200">
            <Save :size="16" />
            {{ saving ? 'Đang lưu...' : 'Lưu thay đổi' }}
          </button>
          <div v-if="editResult" class="p-3 rounded-lg text-sm"
               :class="editResult.success ? 'bg-green-50 dark:bg-green-900/20 text-green-700 dark:text-green-400' : 'bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400'">
            {{ editResult.message }}
          </div>
        </div>
      </div>
    </div>

    <!-- Files Table -->
    <div class="bg-white dark:bg-slate-800 rounded-2xl border border-slate-200 dark:border-slate-700 overflow-hidden">
      <div class="px-6 py-4 border-b border-slate-100 dark:border-slate-700 flex items-center justify-between">
        <h3 class="font-bold text-slate-800 dark:text-white flex items-center gap-2">
          <HardDrive :size="18" class="text-primary" />
          File trên server
          <span class="text-xs bg-slate-100 dark:bg-slate-700 text-slate-500 px-2 py-0.5 rounded-full">{{ files.length }}</span>
        </h3>
      </div>
      <div v-if="files.length === 0" class="px-6 py-12 text-center text-slate-400">
        <CloudUpload :size="40" class="mx-auto mb-3 opacity-50" />
        <p class="text-sm">Chưa có file nào. Upload bản cài đặt đầu tiên ở trên.</p>
      </div>
      <div v-else class="divide-y divide-slate-100 dark:divide-slate-700">
        <div v-for="file in files" :key="file.name"
             class="px-6 py-4 flex items-center gap-4 hover:bg-slate-50 dark:hover:bg-slate-700/30 transition-colors">
          <div class="w-10 h-10 rounded-lg bg-blue-50 dark:bg-blue-900/20 flex items-center justify-center flex-shrink-0">
            <FileDown :size="20" class="text-blue-500" />
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-semibold text-slate-800 dark:text-white truncate">{{ file.name }}</p>
            <p class="text-xs text-slate-400">{{ file.size_mb }} MB · {{ formatDate(file.modified_at) }}</p>
          </div>
          <div class="flex items-center gap-2 flex-shrink-0">
            <a :href="file.url" target="_blank"
               class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-400 hover:text-blue-500 transition-colors"
               title="Tải xuống">
              <Download :size="16" />
            </a>
            <button @click="copyUrl(file.url)"
                    class="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-400 hover:text-primary transition-colors"
                    title="Copy URL">
              <Copy :size="16" />
            </button>
            <button @click="handleDelete(file.name)"
                    class="p-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 text-slate-400 hover:text-red-500 transition-colors"
                    title="Xóa file">
              <Trash2 :size="16" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Upload, Download, Package, RefreshCw, CloudUpload, FileCheck, Rocket, Settings, Save, HardDrive, FileDown, Copy, Trash2 } from 'lucide-vue-next'

definePageMeta({ layout: 'admin' })

const { fetchApi } = useAuth()

const loading = ref(false)
const config = ref({})
const files = ref([])

// Upload state
const selectedFile = ref(null)
const uploadVersion = ref('')
const uploadNotes = ref('')
const uploading = ref(false)
const uploadProgress = ref(0)
const uploadResult = ref(null)
const dragOver = ref(false)

// Edit state
const editVersion = ref('')
const editUrl = ref('')
const editNotes = ref('')
const editSize = ref('')
const saving = ref(false)
const editResult = ref(null)

async function loadData() {
  loading.value = true
  try {
    const r = await fetchApi('/api/admin/update')
    config.value = r.config || {}
    files.value = r.files || []
    // Populate edit fields
    editVersion.value = config.value.latest_version || ''
    editUrl.value = config.value.download_url || ''
    editNotes.value = config.value.release_notes || ''
    editSize.value = config.value.file_size || ''
  } catch (e) {
    console.error('Load update config error:', e)
  } finally {
    loading.value = false
  }
}

function handleFileSelect(e) {
  const file = e.target.files?.[0]
  if (file) {
    selectedFile.value = file
    // Auto-extract version from filename
    const match = file.name.match(/(\d+\.\d+\.\d+)/)
    if (match) uploadVersion.value = match[1]
  }
}

function handleDrop(e) {
  dragOver.value = false
  const file = e.dataTransfer?.files?.[0]
  if (file) {
    selectedFile.value = file
    const match = file.name.match(/(\d+\.\d+\.\d+)/)
    if (match) uploadVersion.value = match[1]
  }
}

async function handleUpload() {
  if (!selectedFile.value) return
  uploading.value = true
  uploadProgress.value = 0
  uploadResult.value = null

  const formData = new FormData()
  formData.append('file', selectedFile.value)
  if (uploadVersion.value) formData.append('version', uploadVersion.value)
  if (uploadNotes.value) formData.append('release_notes', uploadNotes.value)

  try {
    // Use XMLHttpRequest for progress
    const token = localStorage.getItem('admin_token')
    const result = await new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest()
      xhr.open('POST', '/api/admin/update/upload')
      xhr.setRequestHeader('Authorization', `Bearer ${token}`)

      xhr.upload.onprogress = (e) => {
        if (e.lengthComputable) {
          uploadProgress.value = Math.round((e.loaded / e.total) * 100)
        }
      }

      xhr.onload = () => {
        try {
          const data = JSON.parse(xhr.responseText)
          if (xhr.status >= 200 && xhr.status < 300) resolve(data)
          else reject(new Error(data.message || 'Upload failed'))
        } catch { reject(new Error('Parse error')) }
      }

      xhr.onerror = () => reject(new Error('Network error'))
      xhr.send(formData)
    })

    uploadResult.value = { success: true, message: `✅ Upload thành công: v${result.version} — ${result.size_mb}` }
    selectedFile.value = null
    uploadVersion.value = ''
    uploadNotes.value = ''
    await loadData()
  } catch (e) {
    uploadResult.value = { success: false, message: `❌ Lỗi: ${e.message}` }
  } finally {
    uploading.value = false
    uploadProgress.value = 0
  }
}

async function handleEditConfig() {
  saving.value = true
  editResult.value = null
  try {
    const token = localStorage.getItem('admin_token')
    const res = await fetch('/api/admin/update', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
      body: JSON.stringify({
        latest_version: editVersion.value,
        download_url: editUrl.value,
        release_notes: editNotes.value,
        file_size: editSize.value,
      })
    })
    const data = await res.json()
    if (data.success) {
      editResult.value = { success: true, message: '✅ Đã lưu thay đổi' }
      await loadData()
    } else {
      editResult.value = { success: false, message: data.message || 'Lỗi lưu' }
    }
  } catch (e) {
    editResult.value = { success: false, message: `❌ ${e.message}` }
  } finally {
    saving.value = false
  }
}

async function handleDelete(filename) {
  if (!confirm(`Xóa file "${filename}"?`)) return
  try {
    const token = localStorage.getItem('admin_token')
    const res = await fetch(`/api/admin/update/files/${encodeURIComponent(filename)}`, {
      method: 'DELETE',
      headers: { 'Authorization': `Bearer ${token}` }
    })
    const data = await res.json()
    if (data.success) {
      await loadData()
    } else {
      alert(data.message || 'Lỗi xóa file')
    }
  } catch (e) {
    alert(`Lỗi: ${e.message}`)
  }
}

function copyUrl(url) {
  navigator.clipboard.writeText(url)
}

function formatFileSize(bytes) {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1024 / 1024).toFixed(1) + ' MB'
}

function formatDate(str) {
  if (!str) return ''
  try {
    return new Date(str).toLocaleString('vi-VN', { day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' })
  } catch { return str }
}

onMounted(() => loadData())
</script>
