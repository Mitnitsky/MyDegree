<template>
  <b-card no-body
          style="margin: 10px 20px">
    <b-tabs @input="updateActiveSemester"
            card
            pill>
      <b-tab :key="semester.name"
             :title="'סמסטר '+ semester.name"
             lazy
             v-for="semester in this.$store.state.user.semesters">
        <div class="row justify-content-md-center"
        >
          <div class="col-lg-10"
               style="margin-bottom: 10px">
            <app-semester-table :semester="semester"/>
          </div>
          <div class="col-lg-2"
               style="padding: 0 0">
            <app-semester-summary/>
          </div>
        </div>
        <div class="row">
          <div class="col-lg-10">

          </div>
          <div class="col-lg-2"
          >
            <b-button @click="closeTab"
                      class="align-self-end"
                      size="sm"
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
        <b-button @click.prevent="newTab"
                  variant="outline-secondary">הוסף סמסטר
        </b-button>
      </div>
    </b-tabs>
  </b-card>
</template>


<script>
    import AppSemesterSummary from "@/components/SemesterSummary";
    import AppSemesterTable from "@/components/SemesterTable";
    import firebase from 'firebase/app'
    import 'firebase/auth'

    export default {
        name: "semesters-tab-view",
        components: {AppSemesterTable, AppSemesterSummary},
        mounted() {
            let authentication_status = localStorage.getItem('authenticated');
            const user = firebase.auth().currentUser;
            if (user == null) {
                if (authentication_status === 'false') {
                    let user_data = localStorage.getItem('saved_session_data');
                    if (user_data !== null) {
                        if (typeof user_data === 'object') {
                            this.$store.state.user = user_data;
                        } else {
                            this.$store.state.user = JSON.parse(localStorage.getItem('saved_session_data'));
                        }
                    }
                }
            }
        },
        data() {
            return {
                semesters: [],
                tabCounter: 1
            }
        },
        methods: {
            closeTab() {
                this.$store.commit('removeSemester');
            },
            newTab() {
                this.$store.commit('addSemester', 1);
            },
            updateActiveSemester(tab_index) {
                this.$store.commit('changeSemesterTo', tab_index);
                this.$store.commit('reCalcCurrentSemester');
            }
        }
    }
</script>