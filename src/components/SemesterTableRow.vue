<template>
  <tr>
    <td class="courseType">
      <select :select-on-tab="true"
              :value="course.type"
              class="form-control"
              style="text-align-last: center;">
        <option value="0">חובה</option>
        <option value="1">רשימה א'</option>
        <option value="2">רשימה ב'</option>
        <option value="3">הומניסטיים</option>
        <option value="4">בחירה חופשית</option>
        <option value="5">פרוייקט</option>
        <option value="6">ספורט</option>
        <option value="7">פטור</option>
      </select>
    </td>
    <td class="courseNumber">
      <input :value="course.number"
             class="form-control"
             max="9999999"
             min="0"
             step="1"
             style="text-align: center"
             type="number">
    </td>
    <td class="courseName">
      <input :value="course.name"
             class="form-control"
             style="text-align: center"
             type="text">
    </td>
    <td class="coursePoints">
      <input :value="course.points"
             class="form-control"
             max="500"
             min="0"
             step="0.5"
             style="text-align: center"
             type="number">
    </td>
    <td class="courseGrade">
      <input :value="course.grade"
             class="form-control"
             max="100"
             min="0"
             step="1"
             style="text-align: center"
             type="number">
    </td>
    <td class="clearButton text-center">
      <b-button @click="clearRow"
                title="נקה/הסר שורה"
                v-b-tooltip.hover
                variant="outline-secondary">x
      </b-button>
    </td>
  </tr>
</template>
<script>
    //TODO: handle two-way binding
    export default {
        name: 'semester-table-course-row',
        props: ['course', 'index'],
        methods: {
            clearRow() {
                if (this.course.isEmpty()) {
                    this.$store.commit('removeCourse', this.index);
                } else {
                    this.course.clear();
                }
            }
        },
    }
</script>

<style scoped>
  input[type=number]::-webkit-inner-spin-button,
  input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .courseType {

  }

  .courseNumber {

  }

  .courseName {

  }

  .courseGrade {

  }

  .coursePoints {

  }

  .clearButton {

  }
</style>
