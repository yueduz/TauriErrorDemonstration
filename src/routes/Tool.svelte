<script lang="ts">
  import TypeSel from "./TypeSel.svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import { getContext } from "svelte";

  let v = 0;
  async function start() {
    let upd: any = getContext("upd");
    console.log(upd);

    const onEvent = new Channel<number[]>();
    onEvent.onmessage = (message) => {
      // 使用 Uint8Array 从普通数组创建一个 TypedArray
      const typedArray = new Uint8Array(message);

      // 获取 ArrayBuffer
      const arrayBuffer = typedArray.buffer;
      const dataview = new DataView(arrayBuffer);
      // console.log(dataview.getFloat32(3, true));
      v = dataview.getFloat32(0, true);

      upd.fu([v, v, v, v, v, v]);
      // const dataview = new DataView(message);
      // console.log(dataview.getFloat32(0, true));
      // console.log(dataview.getFloat32(3, true));
    };
    invoke("start_fifo", {
      onEvent,
      length: 12 * 60,
    });
  }

  async function stopFifoRead() {
    try {
      await invoke("read_fifo_stop");
    } catch (error) {
      console.error("Failed to stop FIFO read:", error);
    }
  }

  async function read_fifo_pause() {
    try {
      await invoke("read_fifo_pause");
    } catch (error) {
      console.error("Failed to pause FIFO read:", error);
    }
  }
  async function read_fifo_continue() {
    try {
      await invoke("read_fifo_continue");
    } catch (error) {
      console.error("Failed to pause FIFO read:", error);
    }
  }
</script>

<div class="m-3">
  <Button class="opacity-80" variant="destructive" on:click={start}
    >start</Button
  >
  <Button class="opacity-80" on:click={read_fifo_pause}>pause</Button>
  <Button class="opacity-80" on:click={read_fifo_continue}>continue</Button>
  <Button
    class="opacity-80 inline-block"
    variant="destructive"
    on:click={stopFifoRead}>stop</Button
  >
  <TypeSel></TypeSel>
  <Input
    type="number"
    placeholder="数据长度"
    class="max-w-xs inline w-[200px]"
  />
  <span>{v}</span>
</div>
