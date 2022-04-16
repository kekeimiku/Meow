<template>
  <div class="main">
    <el-input
      v-model="input"
      maxlength="640"
      placeholder="示例：\x20\x00\x80\xd2"
      show-word-limit
      :autosize="{ minRows: 7, maxRows: 15 }"
      type="textarea"
    />
    <div style="margin: 20px 0" />
    <div class="button">
      <el-button type="primary" @click="submit">确认</el-button>
      <el-button type="danger" @click="clear">清空</el-button>
    </div>
    <div style="margin: 20px 0" />
    <el-input
      v-model:value="textarea"
      :autosize="{ minRows: 7, maxRows: 15 }"
      type="textarea"
      placeholder="结果:"
    />
    <div style="margin: 20px 0" />暂时只支持arm64
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const createInstance = async () => {
  const response = await fetch('assets/r1.wasm');
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return instance;
};

const write = (string: string, buffer: ArrayBuffer, pointer: number) => {
  console.log(typeof buffer)
  const view = new Uint8Array(buffer, pointer, 2048);
  const encoder = new TextEncoder();
  view.set(encoder.encode(string));
}

const read = (buffer: ArrayBuffer, pointer: number) => {
  const view = new Uint8Array(buffer, pointer, 2048);
  const length = view.findIndex(byte => byte === 0);
  const decoder = new TextDecoder();
  return decoder.decode(new Uint8Array(buffer, pointer, length));
};

const input = ref('')
const textarea = ref('')

async function submit() {
  const instance = await createInstance();
  const memory = instance.exports.memory;
  //@ts-ignore
  const pointer = instance.exports.alloc();
  //@ts-ignore
  write(input.value, memory.buffer, pointer);
  //@ts-ignore
  instance.exports.start(pointer);
  //@ts-ignore
  let r = read(memory.buffer, pointer);
  textarea.value = r;
  //@ts-ignore
  instance.exports.dealloc(pointer);
}

function clear() {
  input.value = '';
  textarea.value = '';
}

</script>

<style>
.main {
  background-color: #fff;
  width: 450px;
  height: 350px;
  margin: auto;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
}
.button {
  text-align: right;
}
</style>