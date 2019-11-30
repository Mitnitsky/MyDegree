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
        <b-button @click.stop="$bvModal.show('modal-center'+'_'+semester.name)"
                  variant="outline-info">חפש קורסים
        </b-button>
        <b-modal
            :header-bg-variant="headerBgVariant"
            :header-text-variant="headerTextVariant"
            :id="'modal-center'+'_'+semester.name"
            centered
            content-class="shadow"
            hide-backdrop
            hide-footer
            ok-title="הוסף קורס"
            title="חיפוש קורסים"
            v-b-modal.modal-scrollable>
          <template v-slot:modal-header="{ close }">
            <div class="row"
                 style="width: 100%">
              <div class="col-11"
                   style="text-align: right;">
                <h5 class="modal-title">חיפוש קורסים</h5>
              </div>
              <div :style="{alignItems: alignment}"
                   class="col-1"
                   style="width: 5%;text-align: left;align-items: flex-end">
                <b-button @click="close()"
                          aria-label="Close"
                          class="close text-light"
                          style="margin-right: 5px;"
                          type="button">×
                </b-button>
              </div>
            </div>
          </template>

          <search-course-dialog/>
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
                headerBgVariant: "dark",
                alignment: "flex-end"
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