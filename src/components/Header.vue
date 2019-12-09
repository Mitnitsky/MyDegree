<template>
  <b-navbar toggleable="sm"
            type="dark"
            variant="dark">
    <b-navbar-toggle target="nav-collapse"/>
    <b-collapse id="nav-collapse"
                is-nav>
      <b-navbar-nav align="start">
        <template v-if="this.logged">
          <b-nav-text href="#"
                      style="font-size: 18px;color: lightgray;margin-left: 10px;">
            שלום {{this.user_name}} !
          </b-nav-text>
          <font-awesome-icon icon="sign-out-alt"
                             size="lg"
                             style="margin-left: 5px;margin-top: 10px;color: lightgray"/>
          <b-nav-item @click="signOut"
                      DIR="ltr"
                      style="font-size: 18px;text-decoration: underline;color: lightgray">יציאה
          </b-nav-item>
        </template>
        <template v-else>
          <font-awesome-icon icon="sign-in-alt"
                             rotation="180"

                             size="lg"
                             style="color: lightgray;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
                             v-b-modal.modal-1/>
          <b-nav-item href="#"
                      style="color: lightgray;text-decoration-line: underline"
                      v-b-modal.modal-1>כניסה
          </b-nav-item>
          <b-modal header-bg-variant="primary"
                   header-text-variant="white"
                   hide-footer
                   hide-header-close

                   id="modal-1"
                   ok-title="סגור"
                   ref="auth-modal"
                   size="md"
                   title="כניסה">
            <authentication/>
            <b-button @click="hideModal"
                      block
                      class="mt-3"
                      variant="outline-primary">סגור
            </b-button>
          </b-modal>

        </template>
        <font-awesome-icon icon="file-import"
                           rotation="180"

                           size="lg"
                           style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
                           v-b-modal.modal-1/>
        <b-nav-item @click="$bvModal.show('modal-import')"
                    href="#"
                    style="font-size: 18px;color: lightgray;text-decoration-line: underline"
        >יבוא קורסים מ-UG
        </b-nav-item>
        <b-modal centered
                 content-class="shadow"
                 header-bg-variant="dark"
                 header-text-variant="white"
                 hide-backdrop
                 hide-footer
                 id="modal-import"
                 ok-title="הוסף קורסים"
                 ok-variant="primary"
                 size="md"
                 title="יבוא קורסים וציונים מ-UG">
          <template v-slot:modal-header="{ close }">
            <div class="row"
                 style="width: 100%">
              <div class="col-lg-11"
                   style="text-align: right;">
                <h5 class="modal-title">יבוא קורסים וציונים מ-UG</h5>
              </div>
              <div class="col-lg-1"
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
          <div class="row justify-content-center">
            <b-button id="popover-button-variant"
                      variant="outline-primary">הוראות
            </b-button>
            <b-popover placement="top"
                       target="popover-button-variant"
                       triggers="hover"
                       variant="outline-dark">
              <template v-slot:title><h4>הוראות</h4></template>
              <p>יש לסמן את כל התוכן באמצעות CTRL+A <a href="https://techmvs.technion.ac.il/cics/wmn/wmngrad?ORD=1"
                                                       target="_blank">באתר ציונים</a> ולהעתיק אותו לתיבת הטקסט בחלון זה
                <br>
                 (<b>אפשרי להעתיק רק את הסמסטרים</b>)
              </p>
            </b-popover>

          </div>
          <div class="row justify-content-center mb-2">
            <b-form-text>
            </b-form-text>
          </div>
          <b-form-textarea id="import-text"
                           placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
                           v-model="message"
          >
          </b-form-textarea>
          <div class="row justify-content-center mt-2">
            <b-button @click="importCourses"
                      variant="outline-primary"
            >
              יבוא קורסים
            </b-button>
          </div>
        </b-modal>
        <font-awesome-icon icon="download"
                           size="lg"
                           style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
        />
        <b-nav-item @click="saveAsJson"
                    href="#"
                    style="font-size: 18px;color: lightgray;text-decoration-line: underline"
                    title="ייצוא מערכת קורסים(ללא ציונים)"
                    v-b-tooltip.hover.v-dark
        >יצוא קורסים ל-JSON
        </b-nav-item>
        <font-awesome-icon icon="upload"
                           size="lg"
                           style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
        />
        <b-nav-item
            href="#"
            style="font-size: 18px;color: lightgray;text-decoration-line: underline"
            v-b-modal.modal-import-from-json
        >יבוא קורסים מ-JSON
        </b-nav-item>
        <b-modal class="modal"
                 centered
                 content-class="shadow"
                 header-bg-variant="dark"
                 header-text-variant="white"
                 hide-backdrop
                 hide-footer
                 id="modal-import-from-json"
                 ok-title="הוסף קורסים"
                 ok-variant="primary"
                 size="md"
                 title="יבוא נתונים מקובץ JSON">
          <template v-slot:modal-header="{ close }">
            <div class="row"
                 style="width: 100%">
              <div class="col-lg-11"
                   style="text-align: right;">
                <h5 class="modal-title">יבוא נתונים מקובץ JSON</h5>
              </div>
              <div class="col-lg-1"
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
          <b-form-textarea id="import-text-json"
                           placeholder="יש להעתיק את התוכן קובץ ה-JSON"
                           v-model="json_text"
          >
          </b-form-textarea>
          <div class="row justify-content-center mt-2">
            <b-button @click="importCoursesFromJSON"
                      variant="outline-primary"
            >
              יבוא קורסים
            </b-button>
          </div>
        </b-modal>
      </b-navbar-nav>
      <b-navbar-nav class="mr-auto">
        <b-navbar-brand href="#"
                        mar
                        style='font-family: "Arial", “Helvetica Neue”, Helvetica, Arial, sans-serif;'>
          Degree Planner
          <img alt=""
               src="../assets/main_icon_white.svg"
               style="width: 48px; height: 48px;margin-right: 5px;"/>
        </b-navbar-brand>
      </b-navbar-nav>
    </b-collapse>

  </b-navbar>
</template>

<script>
    import firebase from "firebase/app"
    import Authentication from "./HeaderAuthentication";
    import {mapFields} from 'vuex-map-fields';
    import 'firebase/auth'
    import 'firebase/firestore'
    import {parseGraduateInformation} from "../store/aux/converter";

    export default {
        components: {Authentication},
        name: "HeaderNavBar",
        computed: {
            ...mapFields([
                'user_name',
                'logged'

            ])
        },
        mounted() {
            firebase.auth().onAuthStateChanged((user) => {
                if (user) {
                    localStorage.setItem('authenticated', 'true');
                    this.logged = true;
                    this.user = user;
                    this.user_name = user.displayName;
                    if (this.$refs['auth-modal']) {
                        this.$refs['auth-modal'].hide();
                    }
                    let uid = firebase.auth().currentUser.uid;
                    firebase.firestore().collection('users').doc(uid).get().then((doc) => {
                        if (doc.exists) {
                            this.$store.commit('fetchUserInfo', doc.data());
                            this.$store.commit('reCalcCurrentSemester');
                        } else {
                            firebase.firestore().collection('users').doc(uid).set(this.$store.state.user).catch(error => {
                                // eslint-disable-next-line no-console
                                console.log('ErrorHeader - ' + error.message);
                            });
                        }
                        window.localStorage.removeItem('saved_session_data');
                    }).catch(error => {
                        // eslint-disable-next-line no-console
                        console.log('ErrorHeader2 - ' + error.message);
                    });
                }
            });
        },
        data() {
            return {
                message: '',
                json_text: ''
            }
        },
        methods: {
            saveAsJson() {
                this.$store.commit('exportSemesters')
            },
            importCourses() {
                if (this.message !== '') {
                    if (confirm('יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?')) {
                        let semesters_exemption = parseGraduateInformation(this.message);
                        this.$store.dispatch('loadUserDataFromUGSite', {
                            "semesters": semesters_exemption['semesters'],
                            "exemption": semesters_exemption['exemption']
                        })
                    }
                }
            },
            importCoursesFromJSON() {
                if (this.json_text !== '') {
                    if (confirm('יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?')) {
                        this.$store.commit('importCoursesFromJson', this.json_text);
                        this.$store.commit('reCalcCurrentSemester');
                    }
                }
            },
            signOut() {
                firebase.auth().signOut();
                localStorage.setItem('authenticated', 'false');
                window.localStorage.removeItem('saved_session_data');
                this.logged = false;
                this.$store.commit('clearUserData');
            },
            hideModal() {
                this.$refs['auth-modal'].hide();
            }
        },
    }
</script>

<style>
  @import "../fonts/Alef/stylesheet.css";

  a.nav-link {
    direction: rtl;
    text-align: start;
  }

  span.navbar-text {
    text-align: start;
    direction: rtl;
  }

  #modal-1___BV_modal_outer_ {
    min-width: 810px !important;
  }
  #modal-1 {

    min-width: 810px !important;

  }
</style>