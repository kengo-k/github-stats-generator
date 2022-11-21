<template>
  <ActiveProjects />
  <UsedLanguages />
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
