<script lang="ts">
  import { onMount } from "svelte";
  import * as monaco from "monaco-editor";
  import { invoke } from "@tauri-apps/api/tauri";

  const uri = monaco.Uri.parse("file:///main");
  const model = monaco.editor.createModel("(con integer 1)", "uplc", uri);
  let result = "";

  async function evaluate() {
    const code = model.getValue();
    result = await invoke("evaluate", { code });
  }

  async function format() {
    result = "";
    try {
      const formattedCode: string = await invoke("format", {
        code: model.getValue(),
      });

      model.setValue(formattedCode);
    } catch (e) {
      result = e;
    }
  }

  onMount(() => {
    const elem = document.getElementById("editor");

    monaco.editor.create(elem, {
      model,
      language: "uplc",
      fontSize: 18,
      theme: "vs-dark",
      automaticLayout: true,
    });
  });
</script>

<div class="container">
  <div class="buttons">
    <button on:click={evaluate}>Eval</button>
    <button on:click={format}>Fmt</button>
    <div />
  </div>

  <div class="panels">
    <div id="editor" />

    <p>{result}</p>
  </div>
</div>

<style>
  .container {
    height: 100%;
    width: 100vw;
    display: flex;
    flex-direction: column;
  }

  .panels {
    display: flex;
    flex-grow: 1;
  }

  #editor {
    width: 50%;
  }

  .buttons {
    display: flex;
  }
</style>
