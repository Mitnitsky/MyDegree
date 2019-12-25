<template>
  <b-navbar toggleable="sm"
            type="dark"
            variant="dark">
    <b-navbar-toggle target="nav-collapse"/>
    <b-collapse id="nav-collapse"
                is-nav>
      <b-navbar-nav align="start">
        <template v-if="this.logged">
          <font-awesome-icon href="#"
                             icon="user-circle"
                             size="lg"
                             style="color: lightgray;margin-right: 0px;margin-left: 0px;font-size: 20px;text-decoration: underline;margin-top:10px"/>
          <b-nav-item-dropdown :text=" this.user_name"
                               right
                               style="font-size: 18px;color: lightgray;">
            <template slot="button-content">שלום <b>{{this.user_name}}</b></template>
            <b-dropdown-item @click="signOut"
                             href="#">
              <font-awesome-icon href="#"
                                 icon="sign-out-alt"
                                 size="lg"
                                 style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"/>
              יציאה
            </b-dropdown-item>
          </b-nav-item-dropdown>

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
            <b-button @click="hideModal('auth-modal')"
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
                 ref="modal-import"
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
                                                       target="_blank">באתר ציונים</a> ולהעתיק אותו לתיבת הטקסט
                 בחלון זה
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
                           no-resize
                           placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
                           rows="5"
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
        <font-awesome-icon icon="sliders-h"
                           size="lg"
                           style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
                           v-b-modal.modal-1/>
        <b-nav-item @click="$bvModal.show('modal-course-types')"
                    href="#"
                    style="font-size: 18px;color: lightgray;text-decoration-line: underline"
        >שינוי קטגוריות קורסים
        </b-nav-item>
        <b-modal centered
                 content-class="shadow"
                 header-bg-variant="dark"
                 header-text-variant="white"
                 hide-backdrop
                 hide-footer
                 id="modal-course-types"
                 ok-disabled
                 ref="modal-course-types"
                 size="md"
                 title="שינוי קטגוריות קורסים">
          <template v-slot:modal-header="{ close }">
            <div class="row"
                 style="width: 100%">
              <div class="col-lg-11"
                   style="text-align: right;">
                <h5 class="modal-title">שינוי קטגוריות קורסים</h5>
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
          <table class="table table-sm ">
            <thead class="thead-dark">
            <tr>
              <th colspan="2"
                  scope="col"
                  style="text-align: center;padding:.3rem">קטגוריות
              </th>
            </tr>
            </thead>
            <tbody>
              <template v-for="(type,index) in course_types">
                <tr :key="index"
                    v-if="index > 3">
                  <td :key="index"
                      class="col-11">
                    <input :value="type.name"
                           class="form-control"
                           type="text">
                  </td>
                  <td class="col-1">
                    <b-button @change="changeCategoryName(index)"
                              @click="deleteCategory(index)"
                              title="מחק קטגוריה"
                              v-b-tooltip.hover.v-secondary
                              variant="outline-danger">x
                    </b-button>
                  </td>
                </tr>
                <tr :key="index"
                    v-else>
                  <td :key="index"
                      colspan="2">
                    <input :value="type.name"
                           class="form-control"
                           readonly
                           style="text-align: center;cursor: default;"
                           type="text">
                  </td>
                </tr>


              </template>
            </tbody>

          </table>
          <div class="row justify-content-center">
            <b-button v-b-modal.modal-add-course-type
                      variant="outline-primary"
            >
              הוסף קטגוריה
            </b-button>
            <b-modal cancel-disabled
                     centered
                     content-class="shadow"
                     header-bg-variant="dark"
                     header-text-variant="white"
                     hide-footer
                     id="modal-add-course-type"
                     ok-disabled
                     size="sm"
                     ref="modal-add-course-type"
                     title="הוספת קטגוריה">
              <template v-slot:modal-header="{ close }">
                <div class="row"
                     style="width: 100%">
                  <div class="col"
                       style="text-align: right;">
                    <h5 class="modal-title">הוספת קטגוריה</h5>
                  </div>
                  <div class="col-2"
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
              <div class="input-group-prepend "
                   style="width: 100%">
                <span class="input-group-text input-group-addon courseNameSpan">שם</span>
                <input @change="invalidInput = false"
                       id="new_category_name"
                       class="form-control  input-group-addon "
                       type="text"
                >
              </div>
              <span dir="rtl"
                    style="size: 12px;color: red;"
                    v-if="wrongInput">קטגוריה עם שם זה קיימת כבר!<br> יש לבחור שם אחר</span>
              <div class="row justify-content-center mt-2">
                <b-button @click="addCategory"
                          variant="outline-primary"
                >
                  הוסף
                </b-button>
              </div>
            </b-modal>
          </div>
        </b-modal>
        <b-nav-item-dropdown id="extra"
                             right>
          <b-dropdown-item @click="exportAsJson"
                           href="#"
                           title="ייצוא מערכת קורסים(ללא ציונים)"
                           v-b-tooltip.hover.left.v-dark>
            <font-awesome-icon href="#"
                               icon="download"
                               size="lg"
                               style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
                               title="ייצוא מערכת קורסים(ללא ציונים)"

            />
            יצוא קורסים לקובץ-JSON
          </b-dropdown-item>
          <b-dropdown-item @click="importCoursesFromJSON"
                           href="#"
                           v-b-modal.modal-import-from-json>
            <font-awesome-icon icon="upload"
                               size="lg"
                               style="color: lightgray;margin-right: 5px;margin-left: 5px;font-size: 20px;text-decoration: underline;margin-top:10px"
            />
            יבוא קורסים מקובץ-JSON
            <b-modal centered
                     class="modal"
                     content-class="shadow"
                     header-bg-variant="dark"
                     header-text-variant="white"
                     hide-backdrop
                     hide-footer
                     id="modal-import-from-json"
                     ok-title="הוסף קורסים"
                     ok-variant="primary"
                     ref="modal-import-from-json"
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
          </b-dropdown-item>
        </b-nav-item-dropdown>

      </b-navbar-nav>
      <b-navbar-nav class="mr-auto">
        <b-navbar-brand href="#"
                        mar
                        style='padding-top: 5px;padding-bottom: 0;margin-left: 5px'>
          My Degree
        </b-navbar-brand>

        <img alt=""
             src="../assets/main_icon_white.svg"
             style="height: 36px;"/>
      </b-navbar-nav>
    </b-collapse>

  </b-navbar>
</template>

<script>
    import firebase from "firebase/app"
    import Authentication from "./HeaderAuthentication";
    import 'firebase/auth'
    import 'firebase/firestore'
    import {parseGraduateInformation} from "../store/aux/converter";
    import {createHelpers} from 'vuex-map-fields';

    const {mapFields} = createHelpers({
        getterType: 'getUserField',
        mutationType: 'updateUserField',
    });

    export default {
        components: {Authentication},
        name: "HeaderNavBar",
        computed: {
            ...mapFields([
                'course_types'
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
                json_text: '',
                user_name: this.$store.state.user_name,
                logged: this.$store.state.logged,
                wrongInput: false
            }
        },
        methods: {
            changeCategoryName(index) {
              this.$store.commit("changeCategoryName", [this.course_types[index].name,index])
            },
            deleteCategory(index) {
              this.$store.commit("deleteCourseType", index);
            },
            addCategory() {
                this.wrongInput = false;
                let new_category_name = document.getElementById("new_category_name").value
                window.console.log(document.getElementById("new_category_name").value)
                for (let course_type of this.course_types) {
                    if (course_type.name === new_category_name) {
                        this.wrongInput = true;
                        return
                    }
                }
                this.$store.commit("addCourseType", new_category_name)
                this.hideModal('modal-add-course-type')
            },
            exportAsJson() {
                this.$store.commit('exportSemesters')
            },
            importCourses() {
                if (this.message !== '') {
                    if (confirm('יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?')) {
                        let semesters_exemption = parseGraduateInformation(this.message);
                        this.$store.dispatch('loadUserDataFromUGSite', {
                            "semesters": semesters_exemption['semesters'],
                            "exemption": semesters_exemption['exemption']
                        });
                        this.message = '';
                        this.hideModal('modal-import');
                    }
                }
            },
            importCoursesFromJSON() {
                if (this.json_text !== '') {
                    if (confirm('יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?')) {
                        this.$store.commit('importCoursesFromJson', this.json_text);
                        this.$store.commit('reCalcCurrentSemester');
                        this.json_text = '';
                        this.hideModal('modal-import-from-json');
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
            hideModal(modalName) {
                if (this.$refs[modalName]) {
                    this.$refs[modalName].hide();
                }
            }
        },
    }
</script>

<style scoped>
  @import "../fonts/Alef/stylesheet.css";


  a.nav-link {
    direction: rtl;
    text-align: start;
  }

  ul {
    padding-inline-start: 10px;
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
