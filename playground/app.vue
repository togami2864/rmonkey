<template>
  <div>
    <NuxtWelcome />
  </div>
</template>

<script lang="ts">
import {output_log} from "rmonkey_wasm"
output_log("hello")
</script>