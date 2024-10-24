<script lang="ts">
  import Tool from "./Tool.svelte";
</script>

<div class="left">
  <Tool></Tool>
</div>

<style lang="css">
  .left {
    position: fixed;  
    top: 0; /* 距离容器顶部的距离 */  
    left: 0; /* 距离容器左侧的距离 */  
    z-index: 2000; /* 较高的 z-index，使其位于 box1 上方 */  

  }
   #mengban {
    background: linear-gradient(160deg, rgba(255, 255, 255, 00), rgba(0, 0, 0, 0.4));  
    pointer-events: none;  

    width: 100vw;
    height: 100vh;
   }
</style>
