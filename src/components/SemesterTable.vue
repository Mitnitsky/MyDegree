<template>
  <div>
    <div class="row">
      <table class="table table-sm table-borderless"
             style="margin-right: 5px; ">
        <semester-header/>
        <tbody>
          <semester-table-row :course="course"
                              :index="index"
                              :key="index"
                              v-for="(course,index) in semester.courses"/>
        </tbody>
      </table>
    </div>
    <div class="row justify-content-md-center"
         style="justify-content: center !important;">
      <b-button-group class="mx-1"
                      style="direction: ltr">

        <b-button @click="removeLastRow"
                  variant="outline-danger">הסר שורה
        </b-button>
        <!--        <b-button @click.stop="$bvModal.show('modal-center'+'_'+semester.name)"-->
        <!--                  variant="info">חיפוש קורסים-->
        <!--        </b-button>-->
        <b-button @click="showModal"
                  variant="info">חיפוש קורסים
        </b-button>
        <!--        <b-modal-->
        <!--            :header-bg-variant="headerBgVariant"-->
        <!--            :header-text-variant="headerTextVariant"-->
        <!--            :id="'modal-center'+'_'+semester.name"-->
        <!--            centered-->
        <!--            content-class="shadow"-->
        <!--            hide-backdrop-->
        <!--            hide-footer-->
        <!--            ok-title="הוסף קורס"-->
        <!--            title="חיפוש קורסים"-->
        <!--            v-b-modal.modal-scrollable>-->
        <modal
            height="auto"
            :clickToClose="false"
            :minHeight=380
            :minWidth=550
            scrollable
            draggable
            name="search">


          <search-course-dialog/>
        </modal>
        <b-button @click="addRow"
                  variant="outline-primary">הוסף שורה
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
            showModal() {
                this.$modal.show('search')
            },
            addRow() {
                this.$store.commit('addCourse');
            },
            removeLastRow() {
                this.$bvModal.msgBoxConfirm('למחוק שורה בעלת תוכן?', {
                    title: 'אזהרה',
                    size: 'sm',
                    headerBgVariant: "dark",
                    headerTextVariant: "white",
                    buttonSize: 'md',
                    cancelDisabled: 'true',
                    okVariant: 'danger',
                    okTitle: 'כן',
                    cancelTitle: 'לא',
                    autoFocusButton: 'ok',
                    footerClass: 'p-2',
                    hideHeaderClose: true,
                    centered: true
                }).then(v => {
                    if (v === true) {
                        this.$store.commit('removeLastRow');
                        this.$store.commit('reCalcCurrentSemester');
                    }
                });
            }
        }
    }
</script>