<script setup lang="ts">
import { ref } from 'vue';
const message = ref(null);

async function getMessage() {
    const response = await fetch('http://localhost:3000/api/get-message');
    const json = await response.json();
    message.value = json.message;
}
</script>

<template>
  <h1>Rust + Vue</h1>
  <div v-if="message">
      Received: {{ message }}
  </div>
  <div v-else>
      No message
  </div>
  <button @click="getMessage()">Get message</button>
</template>