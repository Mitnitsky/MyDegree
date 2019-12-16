<template>
  <thead class="thead-light">
  <tr style="font-family: Alef,serif">
    <th scope="col"
        style="width: 15%;">קטגוריה
    </th>
    <th @click="sortBy('number')"
        class="clickAbleHeader"
        scope="col"
        style="width: 15%;"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary>מספר קורס
    </th>
    <th @click="sortBy('name')"
        class="clickAbleHeader"
        scope="col"
        style="width: 40%;"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary>שם קורס
    </th>
    <th @click="sortBy('points')"
        class="clickAbleHeader"
        scope="col"
        style="width: 12%;"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary>נקודות
    </th>
    <th @click="sortBy('grade')"
        class="clickAbleHeader"
        scope="col"
        style="width: 12%;"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary>ציון
    </th>
    <th scope="col"
        style="width: 6%;"/>
  </tr>
  </thead>
</template>

<script>
    export default {
        name: 'semester-header',
        methods: {
            sortBy(field) {
                this.$store.commit('sortSemesterByField', field)
            }
        }
    }
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