<template>
  <div id="app" class="grid grid-cols-2 h-full">
    <div>
      <ActiveProjects />
    </div>
    <div>
      <UsedLanguages />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ActiveProjects from "./components/ActiveProjects.vue";
import UsedLanguages from "./components/UsedLanguages.vue";

export default defineComponent({
  name: "App",
  components: { ActiveProjects, UsedLanguages },
});
</script>
