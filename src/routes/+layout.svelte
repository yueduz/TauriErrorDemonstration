<script lang="ts">
  import "../app.pcss";
  import LineChart from "./LineChart.svelte";
  import { setContext } from "svelte";
  setContext("upd", {});
</script>

<div>
  <LineChart></LineChart>
  <slot></slot>
</div>

<style lang="css">
  div {
    width: 100vw;
    height: 100vh;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
  }
</style>
