<template>
  <b-card no-body
          style="margin: 10px 20px">
    <b-tabs @input="updateActiveSemester"
            card
            pill>
      <b-tab :key="semester.name"
             :title="'סמסטר '+ semester.name"
             v-for="semester in this.$store.state.user.semesters">
        <div class="row justify-content-md-center">
          <div class="col-lg-8">
            <app-semester-table :semester="semester"/>
          </div>
          <div class="col-lg-2 contaier-fluid"></div>
          <div class="col-lg-2"
               style="padding: 0 0">
            <app-semester-summary/>
          </div>
        </div>
        <div class="row">
          <div class="col-lg-10">

          </div>
          <div class="col-lg-2">
            <b-button @click="closeTab"
                      class="float-right"
                      size="sm"
                      style="margin-bottom:10px"
                      variant="outline-danger">
              מחיקת סמסטר
            </b-button>
          </div>
        </div>
      </b-tab>

      <!-- New Tab Button (Using tabs slot) -->
      <template slot="tabs-end">
        <b-nav-item @click.prevent="newTab"
                    href="#"><b>+</b></b-nav-item>
      </template>

      <!-- Render this if no tabs -->
      <div class="container justify-content-md-center alert alert-secondary text-center text-muted"
           slot="empty">
        <h2>עוד לא נוספו סמסטרים</h2>
        <br>
        הוסף סמטר באמצעות הכפתור <b>+</b> בצד ימין.
      </div>
    </b-tabs>
  </b-card>
</template>


<script>
    import AppSemesterSummary from "@/components/SemesterSummary";
    import AppSemesterTable from "@/components/SemesterTable";


    export default {
        name: "semesters-tab-view",
        components: {AppSemesterTable, AppSemesterSummary},
        data() {
            return {
                semesters: [],
                tabCounter: 1
            }
        },
        methods: {
            closeTab() {
                this.$store.commit('removeSemester');
                this.$store.commit('reCalcCurrentSemester');
            },
            newTab() {
                this.$store.commit('addSemester', 3);
            },
            updateActiveSemester(tab_index) {
                this.$store.commit('changeSemesterTo', tab_index);
                this.$store.commit('reCalcCurrentSemester');
            }
        }
    }
</script>

<style>
  .nav-link: {
    background-color: #2c3e50;
  }
</style>
