<script setup lang="ts">
import { ref, reactive, computed, onMounted, watchEffect, type Ref } from 'vue'

// Local storage setup
const STORAGE_KEY = 'notes'
const notes = ref(JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]')) as Ref<any[]>
watchEffect(() => {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(notes.value))
})

// Get current state
const currentNotes = ref()
const currentNote = reactive({
  id: null,
  title: 'Give me a name',
  content: '',
  tags: [],
  fav: false
})

// Get notes
onMounted(() => {
  allNotes()
})
function allNotes() {
  currentNotes.value = notes.value
}
function favNotes() {
  currentNotes.value = notes.value.filter((note) => note.fav === true)
}

// Dialog logic
const displayDialog = ref(false)
function openDialog() {
  displayDialog.value = true
}
function closeDialog() {
  displayDialog.value = false
}

</script>

<template>
    <div class="flex-grow-1">
      <Panel header="Notes Writer">
        <Toolbar class="mb-6">
          <template #start>
            <Button class="mr-3" label="New" icon="pi pi-plus" @click="addNote" />
            <span class="p-buttonset">
              <Button class="p-button-success" label="All notes" icon="pi pi-list" @click="allNotes" />
              <Button class="p-button-danger" label="Favorites" icon="pi pi-heart" @click="favNotes" />
            </span>
          </template>
          <template #end>
            <!-- Add filter and search functionality here later on -->
          </template>
        </Toolbar>
        <div class="flex flex-wrap justify-content-around gap-3">
          <div class="text-xl" v-if="!notes.length">No notes have been created yet. Hit the <b>New</b> button to create one.</div>
          <Card class="w-3 bg-bluegray-900 shadow-4" v-for="(note, index) in currentNotes" :key="index">
            <template #title>
              {{ note.title }}
            </template>
            <template #subtitle>
              <Tag class="mr-2" :value="tag" v-for="tag in note.tags"></Tag>
            </template>
            <template #content>
              <div class="overflow-hidden max-h-5rem" v-html="note.content"></div>
            </template>
            <template #footer>
              <Button class="p-button-rounded p-button-text" v-tooltip.bottom="'Edit'" icon="pi pi-pencil" @click="editNote(note)" />
              <Button class="p-button-rounded p-button-text p-button-danger" v-tooltip.bottom="'Add to Favorites'" :icon="note.fav ? 'pi pi-heart-fill' : 'pi pi-heart'" @click="note.fav = !note.fav" />
              <Button class="p-button-rounded p-button-text text-red-500" v-tooltip.bottom="'Delete'" icon="pi pi-trash" @click="removeNote(note)" />
            </template>
          </Card>
        </div>
      </Panel>
      <!-- Add Dialog component here later on -->
    </div>
  </template>