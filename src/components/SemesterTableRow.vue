<template>
  <tr>
    <td>
      <select :select-on-tab="true"
              @change.stop="updateField('type')"
              class="form-control courseType"
              v-model.number.lazy="course.type">
        <option :value="types.MUST">חובה</option>
        <option :value="types.LIST_A">רשימה א'</option>
        <option :value="types.LIST_B">רשימה ב'</option>
        <option :value="types.HUMANISTIC">הומניסטיים</option>
        <option :value="types.FREE_CHOICE">בחירה חופשית</option>
        <option :value="types.PROJECTS">פרוייקט</option>
        <option :value="types.SPORT">ספורט</option>
        <option :value="types.EXEMPTION">פטור</option>
      </select>
    </td>
    <td>
      <input @change="updateField('number')"
             class="form-control courseNumber"
             max="9999999"
             min="0"
             step="1"
             type="number"
             v-model.number.lazy="course.number">
    </td>
    <td>
      <input @change="updateField('name')"
             class="form-control courseName"
             type="text"
             v-model.lazy="course.name">
    </td>
    <td>
      <input @change="updateField('points')"
             class="form-control coursePoints"
             max="500"
             min="0"
             step="0.5"
             type="number"
             v-bind:class="[course.points >= 0 ? ''  : InputIsWrong]"
             v-model.number.lazy="course.points">
    </td>
    <td class="">
      <input @change="updateField('grade')"
             class="form-control courseGrade"
             max="100"
             min="0"
             step="1"
             type="number"
             v-bind:class="[course.grade >= 0 && course.grade <= 100 ? ''  : InputIsWrong]"
             v-model.number.lazy="course.grade">
    </td>
    <td class="text-center">
      <b-button @click="clearRow"
                class="clearButton"
                title="נקה/הסר שורה"
                v-b-tooltip.hover.v-secondary
                variant="outline-secondary">x
      </b-button>
    </td>
  </tr>
</template>
<script>
    import {clearCourse, courseIsEmpty} from "../store/classes/course";
    import {course_types} from "../store/classes/course_types";

    export default {
        name: 'semester-table-course-row',
        props: ['course', 'index'],
        data() {
            return {
                types: course_types,
                InputIsWrong: 'inputIsWrong'
            }
        },
        methods: {
            clearRow() {
                if (courseIsEmpty(this.course)) {
                    this.$store.commit('removeCourse', this.index);

                } else {
                    clearCourse(this.course);
                }
                this.$store.commit('reCalcCurrentSemester');
            },
            updateField(field) {
                let value = this.course[field];
                if (field)
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

  .inputIsWrong {
    border-color: red !important;
  }

  .courseType {
    text-align-last: center;
  }

  .courseNumber {
    text-align: center;
    direction: ltr;
  }

  .courseName {
    text-align: center;
    direction: rtl;
  }

  .courseGrade {
    text-align: center;
    direction: ltr
  }

  .coursePoints {
    text-align: center;
    direction: ltr;
  }

  .clearButton {

  }
</style>
