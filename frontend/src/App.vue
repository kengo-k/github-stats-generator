<template>
  <ActiveProjects />
</template>

<script lang="ts">
import { defineComponent } from "vue";
import ActiveProjects from "./components/ActiveProjects.vue";

export default defineComponent({
  name: "App",
  components: { ActiveProjects },
});
</script>
