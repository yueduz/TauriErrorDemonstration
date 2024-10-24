<script lang="ts">
  import { tick } from "svelte";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  const frameworks = [
    {
      value: "int8",
      label: "int8",
    },
    {
      value: "int16",
      label: "int16",
    },
    {
      value: "int32",
      label: "int32",
    },
    {
      value: "int64",
      label: "int64",
    },
    {
      value: "uint8",
      label: "uint8",
    },
    {
      value: "uint16",
      label: "uint16",
    },
    {
      value: "uint32",
      label: "uint32",
    },
    {
      value: "uint64",
      label: "uint64",
    },
    {
      value: "float32",
      label: "float32",
    },
    {
      value: "float64",
      label: "float64",
    }
  ];

  let open = false;
  let value = "";

  $: selectedValue =
    frameworks.find((f) => f.value === value)?.label ?? "type";

  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }
</script>


  
<Popover.Root bind:open let:ids>
  <Popover.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="outline"
      role="combobox"
      aria-expanded={open}
      class="w-[80px] justify-between  bg-white/60"
    >
      {selectedValue}
    </Button>
  </Popover.Trigger>
  <Popover.Content class="w-[80px] p-0">
    <Command.Root>
      <Command.Input placeholder="Search type" class="h-9" />
      <Command.Empty >No framework found.</Command.Empty>
      <Command.Group >
        {#each frameworks as framework}
          <Command.Item
            value={framework.value}
            onSelect={(currentValue) => {
              value = currentValue;
              closeAndFocusTrigger(ids.trigger);
            }}
          >
            {framework.label}
          </Command.Item>
        {/each}
      </Command.Group>
    </Command.Root>
  </Popover.Content>
</Popover.Root>


