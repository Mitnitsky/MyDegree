<template>
  <div class="container-fluid">
    <table class="table table-sm table-borderless"
           style="margin-right: 5px">
      <semester-header/>
      <tbody>
        <semester-table-row :course="course"
                            :index="index"
                            :key="index"
                            v-for="(course,index) in semester.courses"/>
      </tbody>
    </table>
    <div class="row justify-content-md-center">
      <b-button-group class="mx-1"
                      style="direction: ltr">
        <b-button v-b-modal.modal-center
                  variant="outline-info">חפש קורסים
        </b-button>
        <b-modal
            :header-bg-variant="headerBgVariant"
            :header-text-variant="headerTextVariant"
            centered
            content-class="shadow"
            hide-backdrop
            hide-footer
            hide-header-close
            id="modal-center"
            ok-title="הוסף קורס"
            size="md"

            title="חיפוש קורסים">
          <search-course-dialog></search-course-dialog>
        </b-modal>
        <b-button @click="addRow"
                  variant="outline-primary">הוסף שורה
        </b-button>
        <b-button @click="removeLastRow"
                  variant="outline-danger">הסר שורה
        </b-button>
      </b-button-group>
    </div>
  </div>
</template>

<script>
    import SemesterTableRow from "@/components/SemesterTableRow";
    import SemesterHeader from "@/components/SemesterTableHeader"
    import SearchCourseDialog from "./SearchCourseDialog";

    export default {
        name: 'semester-table',
        data() {
            return {
                headerTextVariant: "light",
                headerBgVariant: "dark"
            }
        },
        components: {SemesterTableRow, SemesterHeader, SearchCourseDialog},
        props: {
            semester: null
        },
        methods: {
            addRow() {
                this.$store.commit('addCourse');
            },
            removeLastRow() {
                this.$store.commit('removeLastRow');
                this.$store.commit('reCalcCurrentSemester');
            }
        }
    }
</script>