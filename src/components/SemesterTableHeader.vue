<template>
  <thead class="thead-light">
  <tr style="font-family: Alef,serif">
    <th scope="col"
        style="width: 15%;">קטגוריה
    </th>
    <th scope="col"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary
        @click="sortBy('number')"
        style="width: 15%;">מספר קורס
    </th>
    <th scope="col"
        style="width: 45%;">שם קורס
    </th>
    <th scope="col"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary
        @click="sortBy('points')"
        style="width: 10%;">נקודות
    </th>
    <th scope="col"
        title="לחץ למיון"
        v-b-tooltip.hover.v-secondary
        @click="sortBy('grade')"
        style="width: 10%;">ציון
    </th>
    <th scope="col"
        style="width: 5%;"/>
  </tr>
  </thead>
</template>

<script>
    export default {
        name: 'semester-header',
        methods:{
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
</style>