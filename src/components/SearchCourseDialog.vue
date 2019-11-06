<template>
    <div class="justify-content-md-center">
        <autocomplete
                :get-result-value="getResultValue"
                :search="search"
                aria-label="חיפוש קורסים"
                autoselect
                placeholder="חיפוש קורסים"
                @submit="courseChosen"
                style="direction: rtl"
        ></autocomplete>
        <b-card
                bg-variant="primary" text-variant="white"  class="text-center"
                style="direction: rtl; text-align: right"
                :title="course_name"
                v-if="selected.show"
                :sub-title="course_number"
                v-model="selected.course_name"
               >
                <b-card-header v-model="selected.course_number" style="direction: rtl">
                    {{selected.course_number}}-{{selected.course_name}}
                </b-card-header>
                <b-card-text v-model="selected.course_points" style="direction: rtl">
                    נקודות: {{selected.course_points}}
                </b-card-text>
        </b-card>
    </div>
</template>

<script>
    import Autocomplete from '@trevoreyre/autocomplete-vue'

    const json_courses = require("@/data/courses_2.json");
    export default {
        name: "SearchCourseDialog",

        data() {
            return {
                selected: {
                    show: false,
                    course_name: "",
                    course_number: "",
                    course_points: "",
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
                this.selected.show = false
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
            }
        }
    }
</script>
