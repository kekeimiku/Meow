<template>
  <div class="main">
    <el-input
      v-model="input"
      maxlength="16"
      placeholder="示例：\x20\x00\x80\xd2"
      show-word-limit
      type="text"
    />
    <div style="margin: 20px 0" />
    <div class="button">
      <el-button type="primary" @click="submit">确认</el-button>
      <el-button type="danger">清空</el-button>
    </div>
    <div style="margin: 20px 0" />
    <el-input
      v-model:value="textarea"
      :autosize="{ minRows: 7, maxRows: 15 }"
      type="textarea"
      placeholder="结果:"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'

const input = ref('')
const result = ref('')
const textarea = ref('')

function submit() {
  // console.log(input.value);
  // result.value = input.value;
  (async () => {
    const instance = await createInstance();
    const memory = instance.exports.memory;
    //@ts-ignore
    const pointer = instance.exports.alloc();

    // console.log(input.value);
    //输入
    //@ts-ignore
    write(input.value, memory.buffer, pointer);
    //@ts-ignore
    instance.exports.start(pointer);

    //输出的结果
    //@ts-ignore
    let r = read(memory.buffer, pointer);
    console.log('结果：', r);
    result.value = r;
    textarea.value = textarea.value + result.value + '\n';
    //@ts-ignore
    instance.exports.dealloc(pointer);
  })();
}

const createInstance = async () => {
  const response = await fetch('assets/r0.wasm');
  const bytes = await response.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});

  return instance;
};

const write = (string: any, buffer: any, pointer: any) => {
  const view = new Uint8Array(buffer, pointer, 2048);
  const encoder = new TextEncoder();

  view.set(encoder.encode(string));
}

const read = (buffer: any, pointer: any) => {
  const view = new Uint8Array(buffer, pointer, 2048);
  const length = view.findIndex(byte => byte === 0);
  const decoder = new TextDecoder();

  return decoder.decode(new Uint8Array(buffer, pointer, length));
};


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