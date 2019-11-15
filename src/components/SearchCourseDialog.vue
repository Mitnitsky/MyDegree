<template>
  <div class="justify-content-md-center">
    <autocomplete
        :get-result-value="getResultValue"
        :search="search"
        @submit="courseChosen"
        aria-label="חיפוש קורסים"
        auto-select
        placeholder="חיפוש קורסים"

    ></autocomplete>
    <b-card
        :title="selected_course.name"
        class="text-center"
        style="text-align: right;color: black;margin-top: 7px;  "
        text-variant="white"
        v-if="show"
        v-model="selected_course.name"
    >
      <b-card-header
          style="color: black"
          v-model="selected_course.number">
        <h4>{{selected_course.full_name}}</h4>
      </b-card-header>
      <b-card-text
          style="color: black"
          v-model="selected_course.points">
        <h5 style="text-align: right">נקודות: {{selected_course.points}}</h5>
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected_course.dependencies.length > 0"
          v-model="selected_course.dependencies">
        <h5 style="text-align: right">קורסי קדם:</h5>
        <b-list-group style="margin-bottom: 7px;border-color: #005cbf"
                      v-for="dependencies in selected_course.dependencies"
                      :key="dependencies">
          <b-list-group-item href="#"
                             v-for="depen in dependencies"
                             :key="depen"
                             @click="findDependencies($event)">{{depen}}
          </b-list-group-item>
        </b-list-group>
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected_course.parallel.length > 0"
          v-model="selected_course.parallel">
        <h5 style="text-align: right">קורסים צמודים:</h5>
        <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
          <b-list-group-item href="#"
                             v-for="parallel in selected_course.parallel"
                             :key="parallel">{{parallel}}
          </b-list-group-item>
        </b-list-group>
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected_course.similarities.length > 0"
          v-model="selected_course.similarities">
        <h5 style="text-align: right">קורסים זהים:</h5>
        <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
          <b-list-group-item href="#"
                             v-for="similarities in selected_course.similarities"
                             :key="similarities">{{similarities}}
          </b-list-group-item>
        </b-list-group>
      </b-card-text>
      <b-card-text
          style="color: black"
          v-if="selected_course.inclusive.length > 0"
          v-model="selected_course.inclusive">
        <h5 style="text-align: right">קורסים כלולים:</h5>
        <b-list-group style="margin-bottom: 7px; border-color: #005cbf">
          <b-list-group-item href="#"
                             v-for="inclusive in selected_course.inclusive"
                             :key="inclusive">{{inclusive}}
          </b-list-group-item>
        </b-list-group>
      </b-card-text>

      <b-button type="primary"
                @click="addCourse"
                v-if="show">
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
                show: false,
                selected_course: {
                    full_name: "",
                    name: "",
                    number: "",
                    points: "",
                    dependencies: "",
                    parallel: "",
                    similarities: "",
                    inclusive: "",
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
                return new Promise(resolve => {
                    this.show = false;
                    if (input.length < 1) {
                        resolve([]);
                    }

                    resolve(this.options.filter(course => {
                        return course.full_name.includes(input)
                    }));
                });
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
            findDependencies(event){
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
