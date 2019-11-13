<template>
  <div class="justify-content-md-center">
    <autocomplete
        :get-result-value="getResultValue"
        :search="search"
        @submit="courseChosen"
        aria-label="חיפוש קורסים"
        autoselect="true"
        placeholder="חיפוש קורסים"

    ></autocomplete>
    <b-card
        :sub-title="course_number"
        :title="course_name"

        class="text-center"
        style="text-align: right;color: black;margin-top: 5px;  "
        text-variant="white"
        v-if="selected.show"
        v-model="selected.course_name"
    >
      <b-card-header
          style="color: black"
          v-model="selected.course_number">
        {{selected.course_number}}-{{selected.course_name}}
      </b-card-header>
      <b-card-text
          style="color: black"
          v-model="selected.course_points">
        נקודות: {{selected.course_points}}
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected.course_dependencies != null"
          v-model="selected.course_dependencies">
        קורסי קדם: {{selected.course_dependencies}}
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected.parallel != null"
          v-model="selected.parallel">
        קורסים צמודים: {{selected.parallel}}
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected.similarities != null"
          v-model="selected.similarities">
        קורסים זהים: {{selected.similarities}}
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected.inclusive != null"
          v-model="selected.inclusive">
        קורסים כלולים: {{selected.inclusive}}
      </b-card-text>

      <b-button type="primary"
                v-if="selected.show">
        הוסף קורס
      </b-button>
    </b-card>
  </div>
</template>

<script>
    import Autocomplete from '@trevoreyre/autocomplete-vue'

    const json_courses = require("@/data/courses.json");
    export default {
        name: "SearchCourseDialog",

        data() {
            return {
                selected: {
                    show: false,
                    course_name: "",
                    course_number: "",
                    course_points: "",
                    course_inclusive: "",
                    course_dependencies: "",
                    course_parallel: "",
                    course_similar: ""
                },
                remove: json_courses,
                options: json_courses.courses
            }
        },
        components: {
            Autocomplete
        },
        methods: {
            search(input) {
                this.selected.show = false;
                if (input.length < 1) {
                    return []
                }
                return this.options.filter(course => {
                    return course.course_name
                        .startsWith(input)
                })
            },
            getResultValue(result) {
                return result.course_name
            },
            courseChosen(course) {
                this.selected.show = true;
                this.selected.course_name = course.course_name;
                this.selected.course_number = course.course_number;
                this.selected.course_points = course.points;
                this.selected.course_similar = course.similarities ? course.similarities : null;
                this.selected.course_dependencies = course.dependencies;
                this.selected.course_parallel = course.parallel;
                this.selected.course_inclusive = course.inclusive ? course.inclusive : null;
            }
        }
    }
</script>
