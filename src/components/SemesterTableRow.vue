<template>
  <tr>
    <td class="courseType">
      <select :select-on-tab="true"
              :value="course.type"
              class="form-control"
              style="text-align-last: center;"
              @change.stop="updateField('type', $event.target.value)">
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
             type="number"
             @change.stop="updateField('number', $event.target.value)">
    </td>
    <td class="courseName">
      <input :value="course.name"
             class="form-control"
             style="text-align: center"
             type="text"
             @change.stop="updateField('name', $event.target.value)">
    </td>
    <td class="coursePoints">
      <input :value="course.points"
             class="form-control"
             max="500"
             min="0"
             step="0.5"
             style="text-align: center"
             type="number"
             @change.stop="updateField('points', $event.target.value)">
    </td>
    <td class="courseGrade">
      <input :value="course.grade"
             class="form-control"
             max="100"
             min="0"
             step="1"
             style="text-align: center"
             type="number"
             @change.stop="updateField('grade', $event.target.value)">
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
    import {courseIsEmpty, clearCourse} from "@/store/classes/course";
    //TODO: handle two-way binding
    export default {
        name: 'semester-table-course-row',
        props: ['course', 'index'],
        data() {
            return {
                internalIndex: this.index
            }
        },
        methods: {
            clearRow() {
                if (courseIsEmpty(this.course)) {
                    this.$store.commit('removeCourse', this.index);
                } else {
                    clearCourse(this.course);
                    this.$store.commit('reCalcCurrentSemester');
                }
            },
            updateField(field, value) {
                this.$store.commit('updateCourse', {field, value, index: this.index});
                this.$store.commit('reCalcCurrentSemester');
                this.$store.dispatch('updateSemesterAsync');
            },
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
