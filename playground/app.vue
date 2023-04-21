<template>
  <div class="app">
    <header>
      <button class="button__run" @click="evalRMonkeyCode(input)">Run</button>
    </header>
    <MonacoEditor
      v-model="input"
      class="editor"
      :options="{ minimap: { enabled: false }, theme: 'vs-dark'}"
    />
    <div class="ast"></div>
    <div class="console">
      <p class="result">Result</p>
      <div class="log">
        <div class="output">
          <p  v-for="(res, index) in results" :key="res.currentTime" v-bind:class="index === 0 ? 'eval__result highlight' : 'eval__result'"><span>[{{ res.currentTime }}]</span> <span>{{ res.res }}</span></p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, onMounted } from 'vue';
import { eval_rmonkey } from 'rmonkey_wasm';

const defaultMonkey = `let fibonacci = fn(x) {
  if (x == 0) {
    0;
  } else {
    if (x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};
fibonacci(5);`;

const input = ref(defaultMonkey);
const results = reactive<{ res: string; currentTime: string }[]>([]);

function evalRMonkeyCode(value: string) {
  results.unshift({
    res: eval_rmonkey(value),
    currentTime: getCurrentTimeFormatted(),
  });
}

function getCurrentTimeFormatted() {
  const currentTime = new Date();
  const hours = String(currentTime.getHours()).padStart(2, '0');
  const minutes = String(currentTime.getMinutes()).padStart(2, '0');
  const seconds = String(currentTime.getSeconds()).padStart(2, '0');

  return `${hours}:${minutes}:${seconds}`;
}

onMounted(() => {
  document.addEventListener('keydown', (e) => {
    if (e.key === 's' && (navigator.userAgent.match("Mac") ? e.metaKey : e.ctrlKey)) {
      e.preventDefault()
    }
  })
})
</script>

<style scoped>
.output {
  display: flex;
  flex-direction: column;
}
.container {
  background-color: white;
}
.app {
  height: 100vh;
  width: 100vw;
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  grid-template-rows: 0.1fr repeat(3, 1fr);
  grid-column-gap: 0px;
  grid-row-gap: 0px;
}

header {
  padding: 10px 20px;
  background-color: #202227;
  grid-area: 1 / 1 / 2 / 6;
  display: flex;
  justify-content: space-between;
  border-bottom: 10px solid #15171F;
}
.editor {
  grid-area: 2 / 1 / 5 / 2;
  border-bottom: 1px solid white;
  border-right: 1px solid white;
}
.ast {
  background-color: #15171F;
  grid-area: 2 / 2 / 5 / 3;
}
.console {
  padding: 20px 50px 30px 30px;
  background-color: #15171F;
  grid-area: 5 / 1 / 6 / 3;
  color: #7E7F7F;
  max-height: 200px;
  min-height: 200px;
  border-top: 1px solid #464A54;
}
.log {
  height: calc(100% - 40px);
  scrollbar-base-color: white;
  max-width: 100vw;
  overflow:auto ;
  overflow-anchor: none;
}

footer {
  background-color: black;
  grid-area: 6 / 1 / 7 / 3;
}

.result {
  color: #99999B;
  font-weight: bold;
  padding-bottom: 10px;
}

.button__run {
  background-color: white;
  color: black;
  padding: 10px 20px;
  border-radius: 20px;
  border: solid 2px white;
  font-size: 16px;
  font-weight: bold;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  cursor: pointer;
}
.button__run:hover {
  opacity: 0.7;
  transition: 500ms;
}

.eval__result {
  display:flex;
  justify-content: space-between;
}

.highlight{
  color: lightgreen;
}
</style>
