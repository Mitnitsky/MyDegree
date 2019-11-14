<template>
  <tr>
    <td class="courseType">
      <select :select-on-tab="true"
              v-model.number.lazy="course.type"
              @change.stop="updateField('type')"
              class="form-control"
              style="text-align-last: center;">
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
    <td class="courseNumber">
      <input v-model.number.lazy="course.number"
             @change="updateField('number')"
             class="form-control"
             max="9999999"
             min="0"
             step="1"
             style="text-align: center"
             type="number">
    </td>
    <td class="courseName">
      <input v-model.lazy="course.name"
             @change="updateField('name')"
             class="form-control"
             style="text-align: center"
             type="text">
    </td>
    <td class="coursePoints">
      <input v-model.number.lazy="course.points"
             @change="updateField('points')"
             class="form-control"
             max="500"
             min="0"
             step="0.5"
             style="text-align: center"
             type="number">
    </td>
    <td class="courseGrade">
      <input v-model.number.lazy="course.grade"
             @change="updateField('grade')"
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
    import {clearCourse, courseIsEmpty} from "@/store/classes/course";
    import {course_types} from "../store/classes/course_types";
    //TODO: handle two-way binding
    export default {
        name: 'semester-table-course-row',
        props: ['course', 'index'],
        data() {
            return {
                types: course_types
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
                if(field)
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
