<template>
<div class="app">
  <header>
    <button class="button__run">Run</button>
  </header>
  <div class="editor"></div>
  <div class="ast"></div>
  <div class="console"></div>
  <footer></footer>
</div>
</template>

<script lang="ts">
import {output_log} from "rmonkey_wasm"
output_log("hello")
</script>

<style scoped>
/* .app {
    display: grid;
    grid-template-rows: 60px calc(100% - 290px) 194px 36px;
    height: 100vh;
    min-height: 500px;
    width: 100vw
} */
.container{
background-color: black;
}
.header{
    justify-content: space-between;
}

.app {
  height: 100vh;
display: grid;
grid-template-columns: repeat(2, 1fr);
grid-template-rows: 0.7fr repeat(4, 1fr) 0.5fr;
grid-column-gap: 0px;
grid-row-gap: 0px;
}

.header { grid-area: 1 / 1 / 2 / 6; }
.editor { grid-area: 2 / 1 / 5 / 2; }
.ast { grid-area: 2 / 2 / 5 / 3; }
.console { grid-area: 5 / 1 / 6 / 3; }
.footer { grid-area: 6 / 1 / 7 / 3; }

.button__run {
 background-color: black;
  color: white;
  padding: 10px 20px;
  border-radius: 20px;
  border: none;
  font-size: 16px;
  font-weight: bold;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  cursor: pointer;
}
.button__run:hover {
  background-color: white;
  color: black;
  transition: 500ms;
}

</style>