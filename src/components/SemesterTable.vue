<template>
    <div class="container">
        <table class="table table-bordered" style="margin-right: 5px">
            <semester-header/>
            <tbody>
            <semester-table-course-row :key="i" v-for="i in semester.coursesNumber"/>
            </tbody>
        </table>
        <div class="row justify-content-md-center">
            <b-button-group class="mx-1">
                <b-button squared variant="outline-info" @click="show">Find Course</b-button>
                <modal name="hello-world" :draggable="true">
                    <p>hello, world!</p>
                    <button v-on:click="hide">hide</button>
                </modal>
                <b-button @click="semester.coursesNumber += 1" squared variant="outline-primary">Add Empty Row</b-button>
                <b-button @click="semester.coursesNumber -= 1" squared variant="outline-danger">Remove Row</b-button>
            </b-button-group>
        </div>
    </div>
</template>

<script>
    import SemesterTableCourseRow from "@/components/SemesterTableCourseRow";
    import SemesterHeader from "@/components/SemesterHeader"

    export default {
        name: 'semester-table',
        components: {SemesterTableCourseRow, SemesterHeader},
        props: {
            semester: {}
        },
        methods: {
            show : function() {
                this.$modal.show('hello-world');
            },
        }
    }
</script>