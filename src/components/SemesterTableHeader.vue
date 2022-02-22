<template>
  <thead class="thead-light">
    <tr style="font-family: Alef, serif">
      <th scope="col" style="width: 15%">קטגוריה</th>
      <th
        class="clickAbleHeader"
        scope="col"
        style="width: 15%"
        title="לחץ למיון"
        @click="sortBy('number')"
      >
        מספר קורס
      </th>
      <th
        class="clickAbleHeader"
        scope="col"
        style="width: 40%"
        title="לחץ למיון"
        @click="sortBy('name')"
      >
        שם קורס
      </th>
      <th
        class="clickAbleHeader"
        scope="col"
        style="width: 12%"
        title="לחץ למיון"
        @click="sortBy('points')"
      >
        נקודות
      </th>
      <th
        class="clickAbleHeader"
        scope="col"
        style="width: 12%"
        title="לחץ למיון"
        @click="sortBy('grade')"
      >
        ציון
      </th>
      <th scope="col" style="width: 6%" />
    </tr>
  </thead>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useStore } from "@/use/useStore";
import { USER_STORE } from "@/store/constants";

export default defineComponent({
  name: "SemesterHeader",
  setup() {
    const store = useStore();
    const sortBy = (field: "number" | "grade" | "points" | "name") => {
      store.commit(USER_STORE.MUTATIONS.sortSemesterByField, field);
    };
    return {
      sortBy,
    };
  },
});
</script>

<style scoped>
th {
  color: #495057;
  background-color: #e9ecef;
  border-color: #dee2e6;
  text-align: center;
}

.clickAbleHeader:hover {
  color: cornflowerblue;
  text-decoration: underline;
  cursor: pointer;
}
</style>
