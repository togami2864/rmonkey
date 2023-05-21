<template>
  <div class="App">
    <header>
      <button class="App__EvalCode" @click="evalRMonkeyCode(input)">▶ Run</button>
      <button class="App__EvalCode" @click="formatRMonkey()">📝 Format</button>
    </header>
    <MonacoEditor
      v-model="input"
      class="App__Editor"
      :options="{ ...commonEditorConfig }"
      @input="parse_input_code"
    />
    <MonacoEditor
      v-model="ast"
      class="App_AstViewer"
      :options="{
        ...commonEditorConfig,
        readOnly: true,
      }"
    />
    <div class="App_Console">
      <p class="App__LogHeader">Result</p>
      <div class="App__LogDisplay">
        <p
          v-for="(execData, index) in execResults"
          :key="execData.currentTime"
          v-bind:class="index === 0 ? 'logText highlight' : 'logText'"
        >
          <span>{{ execData.execData }}</span>
          <span>{{
            index === 0
              ? `${execData.duration.toFixed(4)}ms [✨${execData.currentTime}]`
              : `[⌚${execData.currentTime}]`
          }}</span>
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, onMounted } from 'vue';
import { eval_rmonkey, code_to_ast, fmt } from 'rmonkey_wasm';

const commonEditorConfig = {
  minimap: { enabled: false },
  theme: 'vs-dark',
};

const defaultMonkey = `let fibonacci = fn(x) {
  if(x == 0) {
    0;
  } else {
    if(x == 1) {
      1;
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    };
  };
};
fibonacci(5);`;

const input = ref<string>(defaultMonkey);
const ast = ref<string>(parse_code(defaultMonkey));
const execResults = reactive<
  { execData: string; currentTime: string; duration: number }[]
>([
  {
    execData: 'Welcome to rmokey🐒',
    currentTime: getCurrentTimeFormatted(),
    duration: 0,
  },
]);

function evalRMonkeyCode(value: string) {
  const start = performance.now();
  const evaluatedValue = eval_rmonkey(value);
  const end = performance.now();

  execResults.unshift({
    execData: evaluatedValue,
    currentTime: getCurrentTimeFormatted(),
    duration: end - start,
  });
}

function parse_input_code(e: Event) {
  const formattedJson = parse_code(input.value);
  ast.value = formattedJson;
}

function parse_code(input: string): string {
  const code = code_to_ast(input);
  let json;
  try {
    json = JSON.parse(code);
  } catch {
    return code;
  }
  const formattedJson = JSON.stringify(json, null, 2);
  return formattedJson;
}

function formatRMonkey() {
  const formattedCode = fmt(input.value);
  input.value = formattedCode;
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
    if (
      e.key === 's' &&
      (navigator.userAgent.match('Mac') ? e.metaKey : e.ctrlKey)
    ) {
      e.preventDefault();
      formatRMonkey();
    }
  });
});
</script>

<style scoped>
.App {
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
  border-bottom: 10px solid #15171f;
}
.App__Editor {
  grid-area: 2 / 1 / 5 / 2;
  border-bottom: 1px solid white;
  border-right: 1px solid white;
}
.App__AstViewer {
  background-color: #15171f;
  color: white;
  grid-area: 2 / 2 / 5 / 3;
  padding: 5px;
}

.App_Console {
  padding: 20px 50px 30px 30px;
  background-color: #15171f;
  grid-area: 5 / 1 / 6 / 3;
  color: #7e7f7f;
  max-height: 200px;
  min-height: 200px;
  border-top: 1px solid #464a54;
}

.App__LogHeader {
  color: #99999b;
  font-weight: bold;
  padding-bottom: 10px;
  display: flex;
  flex-direction: column;
}

.App__LogDisplay {
  height: calc(100% - 40px);
  scrollbar-base-color: white;
  max-width: 100vw;
  overflow: auto;
  overflow-anchor: none;
}

.logText {
  display: flex;
  justify-content: space-between;
}

.highlight {
  color: lightgreen;
}

.App__EvalCode {
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
  margin: 4px;
}
.App__EvalCode:hover {
  opacity: 0.7;
  transition: 500ms;
}
</style>
