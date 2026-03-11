<template>
  <thead style="background-color: rgb(233, 236, 239)">
    <tr style="font-family: Alef, serif">
      <th scope="col">קטגוריה</th>
      <th
        v-b-tooltip.hover.body
        class="clickAbleHeader"
        scope="col"
        title="לחץ למיון"
        @click="sortBy('number')"
      >
        מספר קורס
      </th>
      <th
        v-b-tooltip.hover.body
        class="clickAbleHeader"
        scope="col"
        title="לחץ למיון"
        @click="sortBy('name')"
      >
        שם קורס
      </th>
      <th
        v-b-tooltip.hover.body
        class="clickAbleHeader"
        scope="col"
        title="לחץ למיון"
        @click="sortBy('points')"
      >
        נקודות
      </th>
      <th
        v-b-tooltip.hover.body
        class="clickAbleHeader"
        scope="col"
        title="לחץ למיון"
        @click="sortBy('grade')"
      >
        ציון
      </th>
      <th scope="col" />
    </tr>
  </thead>
</template>

<script>
export default {
  name: "SemesterHeader",
  methods: {
    sortBy(field) {
      this.$store.commit("sortSemesterByField", field);
    },
  },
};
</script>

<style scoped>
th {
  color: #495057;
  background-color: rgb(233, 236, 239) !important;
  --bs-table-accent-bg: rgb(233, 236, 239) !important;
  border-color: #dee2e6;
  text-align: center;
}

.clickAbleHeader:hover {
  color: cornflowerblue;
  text-decoration: underline;
  cursor: pointer;
}
</style>
