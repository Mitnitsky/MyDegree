<template>
  <div class="justify-content-md-center">
    <autocomplete
        :get-result-value="getResultValue"
        :search="search"
        @submit="courseChosen"
        aria-label="חיפוש קורסים"
        auto-select
        placeholder="חיפוש קורסים"
        style="text-align: right"

    ></autocomplete>
    <b-card
        :header="selected_course.full_name"
        class="text-center"
        style="text-align: right;color: black;margin-top: 7px;  "
        v-if="show"
    >

      <b-list-group style="margin-bottom: 10px">
        <b-list-group-item>נקודות: {{selected_course.points}}</b-list-group-item>
      </b-list-group>

      <b-card header="קורסי קדם:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.prerequisites.length > 0"
              v-model="selected_course.prerequisites">
        <b-list-group

            :key="index"
            v-for="(prerequisites, index) in selected_course.prerequisites">
          <b-list-group-item :key="inner_index"
                             href="#"
                             @click="findPrerequisites($event)"
                             v-for="(course,inner_index) in prerequisites">{{course}}
          </b-list-group-item>
          <p v-if="index < (selected_course.prerequisites.length - 1)" style="margin-bottom: 2px">או-</p>
        </b-list-group>
      </b-card>
      <b-card header="קורסים צמודים:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.linked.length > 0"
              v-model="selected_course.linked">
        <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
          <b-list-group-item :key="linked"
                             href="#"
                             @click="findPrerequisites($event)"
                             v-for="linked in selected_course.linked">{{linked}}
          </b-list-group-item>
        </b-list-group>
      </b-card>
      <b-card header="קורסים ללא זיכוי נוסף:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.overlapping.length > 0"
              v-model="selected_course.overlapping">
        <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
          <b-list-group-item :key="overlapping"
                             href="#"
                             @click="findPrerequisites($event)"
                             v-for="overlapping in selected_course.overlapping">{{overlapping}}
          </b-list-group-item>
        </b-list-group>
      </b-card>
      <b-card header="קורסים כלולים:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.inclusive.length > 0"
              v-model="selected_course.inclusive">
        <b-list-group style="margin-bottom: 7px; border-color: #005cbf">
          <b-list-group-item :key="inclusive"
                             href="#"
                             @click="findPrerequisites($event)"
                             v-for="inclusive in selected_course.inclusive">{{inclusive}}
          </b-list-group-item>
        </b-list-group>
      </b-card>
      <b-card header="קורסים מכילים:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.including.length > 0"
              v-model="selected_course.including">
        <b-list-group style="margin-bottom: 7px; border-color: #005cbf">
          <b-list-group-item :key="including"
                             href="#"
                             @click="findPrerequisites($event)"
                             v-for="including in selected_course.including">{{including}}
          </b-list-group-item>
        </b-list-group>
      </b-card>
      <b-button @click="addCourse"
                type="primary"
                v-if="show">
        הוסף קורס
      </b-button>
    </b-card>
  </div>
</template>

<script>
    import Autocomplete from '@trevoreyre/autocomplete-vue'

    let json_courses;

    if (localStorage.getItem('cources')) {
        json_courses = typeof localStorage.getItem('cources') === 'object' ? localStorage.getItem('cources') : JSON.parse(localStorage.getItem('cources'))
    } else {
        json_courses = require("@/data/courses.json");
        localStorage.setItem('cources', JSON.stringify(json_courses));
    }


    export default {
        name: "SearchCourseDialog",

        data() {
            return {
                show: false,
                selected_course: {
                    full_name: "",
                    name: "",
                    number: "",
                    points: "",
                    prerequisites: "",
                    linked: "",
                    overlapping: "",
                    inclusive: "",
                    including: ""
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
                if (input.length < 2) {
                    return [];
                }
                return this.options.filter(e => e.full_name.includes(input))
            },
            getResultValue(result) {
                return result.full_name
            },
            courseChosen(course) {
                this.show = true;
                this.selected_course = course;
            },
            addCourse() {
                this.$store.commit('addCourseWithData', this.selected_course);
                this.$store.commit('reCalcCurrentSemester');
            },
            findPrerequisites(event) {
                let course_name = event.target.innerText.split(":")[0];
                window.console.log(course_name);
                window.console.log(this.options.filter(course => {
                    return course.full_name.includes(course_name)
                }));
                this.courseChosen(this.options.filter(course => {
                    return course.full_name.includes(course_name)
                })[0])
            }
        }
    }
</script>
