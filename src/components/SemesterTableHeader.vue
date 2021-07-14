<template>
  <thead class="thead-light">
    <tr style="font-family: Alef, serif">
      <th scope="col" style="width: 15%">קטגוריה</th>
      <th
        v-b-tooltip.hover.v-secondary
        class="clickAbleHeader"
        scope="col"
        style="width: 15%"
        title="לחץ למיון"
        @click="sortBy('number')"
      >
        מספר קורס
      </th>
      <th
        v-b-tooltip.hover.v-secondary
        class="clickAbleHeader"
        scope="col"
        style="width: 40%"
        title="לחץ למיון"
        @click="sortBy('name')"
      >
        שם קורס
      </th>
      <th
        v-b-tooltip.hover.v-secondary
        class="clickAbleHeader"
        scope="col"
        style="width: 12%"
        title="לחץ למיון"
        @click="sortBy('points')"
      >
        נקודות
      </th>
      <th
        v-b-tooltip.hover.v-secondary
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
