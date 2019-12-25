<template>
  <div class="justify-content-md-center">
    <autocomplete
        :auto-select="true"
        :get-result-value="getResultValue"
        :search="search"
        @submit="courseChosen"
        aria-label="חיפוש קורסים"
        id="auto-input"
        placeholder="חיפוש קורסים"
        style="text-align: right"

    />
    <b-card
        :header="selected_course.full_name"
        class="text-center"
        header-bg-variant="dark"
        header-text-variant="white"
        style="text-align: right;color: black;margin-top: 7px;  "
        v-if="show"
    >
      <b-card
          header="נקודות:"
          no-body
          style="margin-bottom: 10px; ">
        <p style="margin-top: 5px; margin-bottom: 5px">{{selected_course.points}}</p>
      </b-card>

      <b-card header="קורסי קדם:"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.prerequisites.length > 0"
              v-model="selected_course.prerequisites">
        <b-list-group

            :key="index"
            v-for="(prerequisites, index) in selected_course.prerequisites">
          <b-list-group-item :key="inner_index"
                             :style="{color: checkIfExists(course, 'prerequisite')}"
                             @click="findPrerequisites($event)"
                             href="#"
                             v-for="(course,inner_index) in prerequisites">{{course}}
          </b-list-group-item>
          <p style="margin-bottom: 2px"
             v-if="index < (selected_course.prerequisites.length - 1)">או-</p>
        </b-list-group>
      </b-card>
      <b-card header="קורסים צמודים:"
              header-bg-variant="dark"
              header-text-variant="white"
              no-body
              style="margin-bottom: 10px"
              v-if="selected_course.linked.length > 0"
              v-model="selected_course.linked">
        <b-list-group style="margin-bottom: 7px;">
          <b-list-group-item :key="linked"
                             :style="{color: checkIfExists(linked, 'linked')}"
                             @click="findPrerequisites($event)"
                             href="#"
                             v-for="linked in selected_course.linked">{{linked}}
          </b-list-group-item>
        </b-list-group>
      </b-card>
      <div class="row justify-content-center">
        <b-button @click="addCourse"
                  type="primary"
                  v-if="show">
          הוסף קורס
        </b-button>
      </div>
      <b-button @click="collapsed = !collapsed"
                style="margin: 5px;"
                v-b-toggle.collapse-additional-info
                v-if="collapsed"
                variant="outline-secondary">הראה מידע נוסף &Darr;
      </b-button>
      <b-button @click="collapsed = !collapsed"
                style="margin: 5px"
                v-b-toggle.collapse-additional-info
                v-if="!collapsed"
                variant="outline-secondary">הסתר מידע נוסף &Uarr;
      </b-button>
      <b-collapse id="collapse-additional-info">
        <b-card header="קורסים ללא זיכוי נוסף:"
                no-body
                style="margin-bottom: 10px"
                v-if="selected_course.overlapping.length > 0"
                v-model="selected_course.overlapping">
          <b-list-group style="margin-bottom: 7px;border-color: #005cbf">
            <b-list-group-item :key="overlapping"
                               :style="{color: checkIfExists(overlapping, 'other')}"
                               @click="findPrerequisites($event)"
                               href="#"
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
                               :style="{color: checkIfExists(inclusive, 'other')}"
                               @click="findPrerequisites($event)"
                               href="#"
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
                               :style="{color: checkIfExists(including, 'other')}"
                               @click="findPrerequisites($event)"
                               href="#"
                               v-for="including in selected_course.including">{{including}}
            </b-list-group-item>
          </b-list-group>
        </b-card>
      </b-collapse>
    </b-card>
  </div>
</template>

<script>
    import Autocomplete from '@trevoreyre/autocomplete-vue'

    let json_courses;

    if (localStorage.getItem('courses')) {
        json_courses = typeof localStorage.getItem('courses') === 'object' ? localStorage.getItem('courses') : JSON.parse(localStorage.getItem('courses'))
        if(!json_courses.version || json_courses.version <= 1.0){
            json_courses = require("../data/courses.json");
            localStorage.setItem('courses', JSON.stringify(json_courses));
        }
    } else {
        json_courses = require("../data/courses.json");
        localStorage.setItem('courses', JSON.stringify(json_courses));
    }


    export default {
        name: "SearchCourseDialog",

        data() {
            return {
                show: false,
                collapsed: true,
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
                if (!(this.selected_course.name.includes("ספורט") || this.selected_course.name.includes("גופני"))) {
                    let course_number_and_anwser = {course_number: this.selected_course.number, answer: ''};
                    this.$store.commit('checkIfCourseExists', course_number_and_anwser);
                    if (course_number_and_anwser.answer !== false) {
                        let message = "הקורס קיים בסמסטר " + course_number_and_anwser.answer + ", להוסיף בכל זאת?";
                        if (confirm(message)) {
                            this.$store.commit('addCourseWithData', this.selected_course);
                            this.$store.commit('reCalcCurrentSemester');
                        }
                    } else {
                        this.$store.commit('addCourseWithData', this.selected_course);
                        this.$store.commit('reCalcCurrentSemester');
                    }
                }else{
                    this.$store.commit('addCourseWithData', this.selected_course);
                    this.$store.commit('reCalcCurrentSemester');
                }
            },
            findPrerequisites(event) {
                let course_name = event.target.innerText.split(":")[0];
                this.courseChosen(this.options.filter(course => {
                    return course.full_name.includes(course_name)
                })[0]);
            },
            checkIfExists(course_full_name, type) {
                let course_name = course_full_name.split(":")[1];
                if (course_name.includes('השלמות')) {
                    return 'black';
                }
                let course_number = course_full_name.split(":")[0];
                let course_number_and_answer = {course_number: course_number, answer: ''};
                if (type === 'prerequisite') {
                    this.$store.commit('checkPrerequisites', course_number_and_answer)
                } else if (type === 'linked') {
                    this.$store.commit('checkLinear', course_number_and_answer);
                } else {
                    this.$store.commit('checkIfCourseExists', course_number_and_answer);
                    //It's bad if one of inclusive/including/similar courses are in the table
                    return course_number_and_answer.answer === true ? 'red' : 'black';
                }
                return course_number_and_answer.answer === false ? 'red' : 'black';
            }
        }
    }
</script>