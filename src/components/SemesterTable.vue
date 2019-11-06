<template>
    <div class="container">
        <table class="table table-bordered" style="margin-right: 5px">
            <semester-header/>
            <tbody>
            <semester-table-course-row :key="i" v-for="i in semester.coursesNumber"/>
            </tbody>
        </table>
        <div class="row justify-content-md-center">
            <b-button-group class="mx-1" >
                <b-button v-b-modal.modal-center squared variant="outline-info">Find Course</b-button>
                <b-modal
                        size="xl"
                        id="modal-center"
                        hide-backdrop
                        centered
                        title="חיפוש קורסים"
                        ok-title="הוסף קורס"
                        cancel-title="סגור">
                    <search-course-dialog></search-course-dialog>
                </b-modal>
                <b-button @click="semester.coursesNumber += 1" squared variant="outline-primary">Add Empty Row
                </b-button>
                <b-button @click="semester.coursesNumber -= 1" squared variant="outline-danger">Remove Row</b-button>
            </b-button-group>
        </div>
    </div>
</template>

<script>
    import SemesterTableCourseRow from "@/components/SemesterTableCourseRow";
    import SemesterHeader from "@/components/SemesterHeader"
    import SearchCourseDialog from "./SearchCourseDialog";

    export default {
        name: 'semester-table',
        components: {SemesterTableCourseRow, SemesterHeader, SearchCourseDialog},
        props: {
            semester: {}
        },
        methods: {
        }
    }
</script>