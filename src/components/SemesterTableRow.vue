<template>
  <tr>
    <td>
      <select :select-on-tab="true"
              @change.stop="updateField('type')"
              class="form-control courseType"
              v-model.number.lazy="course.type">
        <template v-for="(type, index) in course_types" >
           <option :value="index" v-bind:key="index">{{type.name}}</option>
        </template>

      </select>
    </td>
    <td style="min-width: 90px">
      <input @change="updateField('number')"
             class="form-control courseNumber"
             max="9999999"
             min="0"
             step="1"
             type="number"
             v-model.number.lazy="course.number">
    </td>
    <td style="min-width: 250px;padding-right: 0">
      <input @change="updateField('name')"
             class="form-control courseName"
             type="text"
             v-model.lazy="course.name">
    </td>
    <td style="min-width: 60px">
      <input @change="updateField('points')"
             class="form-control coursePoints"
             max="500"
             min="0"
             step="0.5"
             type="number"
             v-bind:class="[course.points >= 0 ? ''  : InputIsWrong]"
             v-model.number.lazy="course.points">
    </td>
    <td style="min-width: 60px">
      <input @change="updateField('grade')"
             class="form-control courseGrade"
             max="100"
             min="0"
             step="1"
             type="number"
             v-bind:class="[course.grade >= 0 && course.grade <= 100 ? ''  : InputIsWrong]"
             v-model.number.lazy="course.grade">
    </td>
    <td class="text-center"
        style="min-width: 45px">
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
    import {createHelpers} from 'vuex-map-fields';

    const {mapFields} = createHelpers({
        getterType: 'getUserField',
        mutationType: 'updateUserField',
    });

    export default {
        name: 'semester-table-course-row',
        props: ['course', 'index'],
        data() {
            return {
                types: course_types,
                InputIsWrong: 'inputIsWrong'
            }
        },
        computed: {
            ...mapFields([
                'course_types'
            ]),
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
