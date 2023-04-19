<template>
    <div class="app">
        <header>
            <button class="button__run">Run</button>
        </header>
        <div class="editor"></div>
        <div class="ast"></div>
        <div class="console">
            <p class="result">Result</p>
        </div>
        <footer></footer>
    </div>
</template>

<script lang="ts">
import { output_log } from "rmonkey_wasm";
output_log("hello");
</script>

<style scoped>
.container {
    background-color: black;
}
.header {
    justify-content: space-between;
}

.app {
    height: 100vh;
    width: 100vw;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: 0.4fr repeat(4, 1fr) 0.3fr;
    grid-column-gap: 0px;
    grid-row-gap: 0px;
}

header {
    padding: 10px 20px;
    background-color: black;
    grid-area: 1 / 1 / 2 / 6;
    display: flex;
    justify-content: space-between;
}
.editor {
    background-color: #171417;
    grid-area: 2 / 1 / 5 / 2;
    border-bottom: 1px solid white;
    border-right: 1px solid white;

}
.ast {
    background-color: #171417;
    grid-area: 2 / 2 / 5 / 3;
    border-bottom: 1px solid white;

}
.console {
    background-color: #171417;
    grid-area: 5 / 1 / 6 / 3;
}
footer {
    background-color: black;
    grid-area: 6 / 1 / 7 / 3;
}

.result{
    color: white;
    font-weight: bold;
    padding: 10px;
}

.button__run {
    background-color: black;
    color: white;
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
    background-color: white;
    color: black;
    transition: 500ms;
}
</style>
