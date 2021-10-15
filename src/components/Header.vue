<template>
  <b-navbar toggleable="sm" type="dark" variant="dark">
    <b-navbar-toggle target="nav-collapse" />
    <b-collapse id="nav-collapse" is-nav>
      <b-navbar-nav align="start">
        <template v-if="logged">
          <font-awesome-icon
            href="#"
            icon="user-circle"
            size="lg"
            style="
              color: lightgray;
              margin-right: 0;
              margin-left: 0;
              font-size: 20px;
              margin-top: 10px;
            "
          />
          <b-nav-item-dropdown
            :text="user_name"
            right
            style="font-size: 18px; color: lightgray"
          >
            <template slot="button-content">
              <span style="margin-left: 5px">שלום {{ user_name }}</span>
            </template>
            <b-dropdown-item href="#" @click="signOut">
              <font-awesome-icon
                href="#"
                icon="sign-out-alt"
                size="lg"
                style="
                  color: lightgray;
                  margin-right: 5px;
                  margin-left: 5px;
                  font-size: 20px;
                  margin-top: 10px;
                "
                @click="signOut"
              />
              יציאה
            </b-dropdown-item>
          </b-nav-item-dropdown>
        </template>
        <template v-else>
          <font-awesome-icon
            v-b-modal.modal-1
            icon="sign-in-alt"
            rotation="180"
            size="lg"
            style="
              color: lightgray;
              margin-left: 5px;
              font-size: 20px;
              margin-top: 10px;
            "
          />
          <b-nav-item v-b-modal.modal-1 href="#" style="color: lightgray">
            כניסה
          </b-nav-item>
          <b-modal
            id="modal-1"
            ref="auth-modal"
            header-bg-variant="primary"
            header-text-variant="white"
            hide-footer
            hide-header-close
            ok-title="סגור"
            size="md"
            title="כניסה"
          >
            <authentication />
            <b-button
              block
              class="mt-3"
              variant="outline-primary"
              @click="hideModal('auth-modal')"
            >
              סגור
            </b-button>
          </b-modal>
        </template>
        <font-awesome-icon
          v-b-modal.modal-import-from-ug
          icon="file-import"
          rotation="180"
          size="lg"
          style="
            color: lightgray;
            margin-right: 5px;
            margin-left: 5px;
            font-size: 20px;
            margin-top: 10px;
          "
        />
        <b-nav-item
          href="#"
          style="font-size: 18px; color: lightgray"
          @click="$bvModal.show('modal-import-from-ug')"
        >
          יבוא קורסים מ-UG
        </b-nav-item>
        <b-modal
          id="modal-import-from-ug"
          ref="modal-import-from-ug"
          centered
          content-class="shadow"
          header-bg-variant="dark"
          header-text-variant="white"
          hide-backdrop
          hide-footer
          ok-title="הוסף קורסים"
          ok-variant="primary"
          size="md"
          title="יבוא קורסים וציונים מ-UG"
        >
          <template #modal-header="{ close }">
            <div class="row" style="width: 100%">
              <div class="col-lg-11" style="text-align: right">
                <h5 class="modal-title">יבוא קורסים וציונים מ-UG</h5>
              </div>
              <div
                class="col-lg-1"
                style="width: 5%; text-align: left; align-items: flex-end"
              >
                <b-button
                  aria-label="Close"
                  class="close text-light"
                  style="margin-right: 5px"
                  type="button"
                  @click="close()"
                >
                  ×
                </b-button>
              </div>
            </div>
          </template>
          <div class="row justify-content-center">
            <b-button id="popover-button-variant" variant="outline-primary">
              הוראות
            </b-button>
            <b-popover
              placement="top"
              target="popover-button-variant"
              triggers="hover"
              variant="outline-dark"
            >
              <template #title>
                <h4>הוראות</h4>
              </template>
              <p>
                יש לסמן את כל התוכן באמצעות CTRL+A
                <a
                  href="https://techmvs.technion.ac.il/cics/wmn/wmngrad?ORD=1"
                  target="_blank"
                  >באתר ציונים</a
                >
                ולהעתיק אותו לתיבת הטקסט בחלון זה
                <br />
                (<b>אפשרי להעתיק רק את הסמסטרים</b>)
              </p>
            </b-popover>
          </div>
          <div class="row justify-content-center mb-2">
            <b-form-text />
          </div>
          <b-form-textarea
            id="import-text"
            v-model="message"
            no-resize
            placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
            rows="5"
          />
          <div class="row justify-content-center mt-2">
            <b-button variant="outline-primary" @click="importCourseFromUG">
              יבוא קורסים
            </b-button>
          </div>
        </b-modal>
        <font-awesome-icon
          v-b-modal.modal-import-from-students
          icon="file-import"
          rotation="180"
          size="lg"
          style="
            color: lightgray;
            margin-right: 5px;
            margin-left: 5px;
            font-size: 20px;
            margin-top: 10px;
          "
        />
        <b-nav-item
          href="#"
          style="font-size: 18px; color: lightgray"
          @click="$bvModal.show('modal-import-from-students')"
        >
          יבוא קורסים מ-Students
        </b-nav-item>
        <b-modal
          id="modal-import-from-students"
          ref="modal-import-from-students"
          centered
          content-class="shadow"
          header-bg-variant="dark"
          header-text-variant="white"
          hide-backdrop
          hide-footer
          ok-title="הוסף קורסים"
          ok-variant="primary"
          size="md"
          title="יבוא קורסים וציונים מ-Students"
        >
          <template #modal-header="{ close }">
            <div class="row" style="width: 100%">
              <div class="col-lg-11" style="text-align: right">
                <h5 class="modal-title">יבוא קורסים וציונים מ-Students</h5>
              </div>
              <div
                class="col-lg-1"
                style="width: 5%; text-align: left; align-items: flex-end"
              >
                <b-button
                  aria-label="Close"
                  class="close text-light"
                  style="margin-right: 5px"
                  type="button"
                  @click="close()"
                >
                  ×
                </b-button>
              </div>
            </div>
          </template>
          <div class="row justify-content-center">
            <b-button id="popover-button-variant" variant="outline-primary">
              הוראות
            </b-button>
            <b-popover
              placement="top"
              target="popover-button-variant"
              triggers="hover"
              variant="outline-dark"
            >
              <template #title>
                <h4>הוראות</h4>
              </template>
              <p>
                יש לסמן את כל התוכן באמצעות CTRL+A
                <a
                  href="https://students.technion.ac.il/local/tcurricular/grades"
                  target="_blank"
                  >באתר ציונים</a
                >
                ולהעתיק אותו לתיבת הטקסט בחלון זה
                <br />
                (<b>אפשרי להעתיק רק את הסמסטרים</b>)
              </p>
            </b-popover>
          </div>
          <div class="row justify-content-center mb-2">
            <b-form-text />
          </div>
          <b-form-textarea
            id="import-text"
            v-model="message"
            no-resize
            placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
            rows="5"
          />
          <div class="row justify-content-center mt-2">
            <b-button
              variant="outline-primary"
              @click="importCourseFromStudents"
            >
              יבוא קורסים
            </b-button>
          </div>
        </b-modal>

        <!--        --><!--        --><!--        --><!--        -->
        <font-awesome-icon
          v-b-modal.modal-cf-import
          icon="file-import"
          rotation="180"
          size="lg"
          style="
            color: lightgray;
            margin-right: 5px;
            margin-left: 5px;
            font-size: 20px;
            margin-top: 10px;
          "
        />
        <b-nav-item
          href="#"
          style="font-size: 18px; color: lightgray"
          @click="$bvModal.show('modal-cf-import')"
        >
          יבוא סמסטר מ-CheeseFork
        </b-nav-item>
        <b-modal
          id="modal-cf-import"
          ref="modal-cf-import"
          centered
          content-class="shadow"
          header-bg-variant="dark"
          header-text-variant="white"
          hide-backdrop
          hide-footer
          ok-title="הוסף קורסים"
          ok-variant="primary"
          size="md"
          title="יבוא סמסטר מ-CheeseFork"
        >
          <template #modal-header="{ close }">
            <div class="row" style="width: 100%">
              <div class="col-lg-11" style="text-align: right">
                <h5 class="modal-title">יבוא סמסטר מ-CheeseFork</h5>
              </div>
              <div
                class="col-lg-1"
                style="width: 5%; text-align: left; align-items: flex-end"
              >
                <b-button
                  aria-label="Close"
                  class="close text-light"
                  style="margin-right: 5px"
                  type="button"
                  @click="close()"
                >
                  ×
                </b-button>
              </div>
            </div>
          </template>
          <div class="row justify-content-center">
            <b-button id="popover-cf-variant" variant="outline-primary">
              הוראות
            </b-button>
            <b-popover
              placement="top"
              target="popover-cf-variant"
              triggers="hover"
              variant="outline-dark"
            >
              <template #title>
                <h4>הוראות</h4>
              </template>
              <p>
                יש לסמן את הקורסים<a
                  href="https://cheesefork.cf/"
                  target="_blank"
                  >Cheesefork</a
                >
                ולהעתיק אותו לתיבת הטקסט בחלון זה
              </p>
              <img src="../../images/import_from_cf.png" />
            </b-popover>
          </div>
          <div class="row justify-content-center mb-2">
            <b-form-text />
          </div>
          <b-form-textarea
            id="import-text"
            v-model="input_data"
            no-resize
            placeholder="יש להעתיק את התוכן לכאן"
            rows="5"
          />
          <div class="row justify-content-center mt-2">
            <b-button variant="outline-primary" @click="importCoursesFromCF">
              יבוא קורסים
            </b-button>
          </div>
        </b-modal>

        <font-awesome-icon
          v-b-modal.modal-course-types
          icon="sliders-h"
          size="lg"
          style="
            color: lightgray;
            margin-right: 5px;
            margin-left: 5px;
            font-size: 20px;
            margin-top: 10px;
          "
        />
        <b-nav-item
          href="#"
          style="font-size: 18px; color: lightgray"
          @click="$bvModal.show('modal-course-types')"
        >
          שינוי קטגוריות קורסים
        </b-nav-item>
        <b-modal
          id="modal-course-types"
          ref="modal-course-types"
          centered
          content-class="shadow"
          header-bg-variant="dark"
          header-text-variant="white"
          hide-backdrop
          hide-footer
          ok-disabled
          size="md"
          title="שינוי קטגוריות קורסים"
        >
          <template #modal-header="{ close }">
            <div class="row" style="width: 100%">
              <div class="col-lg-11" style="text-align: right">
                <h5 class="modal-title">שינוי קטגוריות קורסים</h5>
              </div>
              <div
                class="col-lg-1"
                style="width: 5%; text-align: left; align-items: flex-end"
              >
                <b-button
                  class="close text-light"
                  style="margin-right: 5px"
                  type="button"
                  aria-label="Close"
                  @click="close()"
                >
                  ×
                </b-button>
              </div>
            </div>
          </template>
          <table class="table table-sm">
            <thead class="thead-dark">
              <tr>
                <th
                  colspan="2"
                  scope="col"
                  style="text-align: center; padding: 0.3rem"
                >
                  קטגוריות
                </th>
              </tr>
            </thead>
            <tbody>
              <template v-for="(type, index) in course_types">
                <tr
                  v-if="type.name === 'פטור' || type.name === 'חובה'"
                  :key="index"
                >
                  <td :key="index" colspan="2">
                    <input
                      :value="type.name"
                      class="form-control"
                      readonly
                      style="text-align: center; cursor: default"
                      type="text"
                    />
                  </td>
                </tr>
                <tr
                  v-if="
                    index > 0 && type.name !== 'פטור' && type.name !== 'חובה'
                  "
                  :key="index"
                >
                  <td :key="index" class="col-11">
                    <input
                      :value="type.name"
                      class="form-control"
                      type="text"
                      @input="changeCategoryName(index, $event)"
                    />
                  </td>
                  <td class="col-1">
                    <b-button
                      v-b-tooltip.hover.v-secondary
                      title="מחק קטגוריה"
                      variant="outline-danger"
                      @click="deleteCategory(index)"
                    >
                      x
                    </b-button>
                  </td>
                </tr>
              </template>
            </tbody>
          </table>
          <div class="row justify-content-center">
            <b-button v-b-modal.modal-add-course-type variant="outline-primary">
              הוסף קטגוריה
            </b-button>
            <b-modal
              id="modal-add-course-type"
              ref="modal-add-course-type"
              cancel-disabled
              centered
              content-class="shadow"
              header-bg-variant="dark"
              header-text-variant="white"
              hide-footer
              ok-disabled
              size="sm"
              title="הוספת קטגוריה"
            >
              <template #modal-header="{ close }">
                <div class="row" style="width: 100%">
                  <div class="col" style="text-align: right">
                    <h5 class="modal-title">הוספת קטגוריה</h5>
                  </div>
                  <div
                    class="col-2"
                    style="width: 5%; text-align: left; align-items: flex-end"
                  >
                    <b-button
                      aria-label="Close"
                      class="close text-light"
                      style="margin-right: 5px"
                      type="button"
                      @click="close()"
                    >
                      ×
                    </b-button>
                  </div>
                </div>
              </template>
              <div class="input-group-prepend" style="width: 100%">
                <input
                  id="new_category_name"
                  class="form-control input-group-addon"
                  placeholder="שם קטגוריה"
                  type="text"
                  @input="hideInvalidInput"
                />
              </div>
              <span
                v-if="wrongInput"
                id="invalid-input"
                dir="rtl"
                style="size: 12px; color: red"
                >קטגוריה עם שם כזה קיימת כבר!</span
              >
              <div class="row justify-content-center mt-2">
                <b-button variant="outline-primary" @click="addCategory">
                  הוסף
                </b-button>
              </div>
            </b-modal>
          </div>
        </b-modal>
        <b-nav-item-dropdown
          id="extra"
          text="..."
          style="font-size: 18px; color: lightgray"
          right
        >
          <b-dropdown-item
            v-b-tooltip.hover.left.v-dark
            href="#"
            title="ייצוא מערכת קורסים(ללא ציונים)"
            @click="exportAsJson"
          >
            <font-awesome-icon
              href="#"
              icon="download"
              size="lg"
              style="
                color: lightgray;
                margin-right: 5px;
                margin-left: 5px;
                font-size: 20px;
                margin-top: 10px;
              "
              title="ייצוא מערכת קורסים(ללא ציונים)"
            />
            יצוא קורסים לקובץ-JSON
          </b-dropdown-item>
          <b-dropdown-item v-b-modal.modal-import-from-json href="#">
            <font-awesome-icon
              v-b-modal.modal-import-from-json
              icon="upload"
              size="lg"
              style="
                color: lightgray;
                margin-right: 5px;
                margin-left: 5px;
                font-size: 20px;
                margin-top: 10px;
              "
            />
            יבוא קורסים מקובץ-JSON
            <b-modal
              id="modal-import-from-json"
              ref="modal-import-from-json"
              centered
              class="modal"
              content-class="shadow"
              header-bg-variant="dark"
              header-text-variant="white"
              hide-backdrop
              hide-footer
              ok-title="הוסף קורסים"
              ok-variant="primary"
              size="md"
              title="יבוא נתונים מקובץ JSON"
            >
              <template #modal-header="{ close }">
                <div class="row" style="width: 100%">
                  <div class="col-lg-11" style="text-align: right">
                    <h5 class="modal-title">יבוא נתונים מקובץ JSON</h5>
                  </div>
                  <div
                    class="col-lg-1"
                    style="width: 5%; text-align: left; align-items: flex-end"
                  >
                    <b-button
                      aria-label="Close"
                      class="close text-light"
                      style="margin-right: 5px"
                      type="button"
                      @click="close()"
                    >
                      ×
                    </b-button>
                  </div>
                </div>
              </template>
              <b-form-textarea
                id="import-text-json"
                v-model="json_text"
                placeholder="יש להעתיק את התוכן קובץ ה-JSON"
              />
              <div class="row justify-content-center mt-2">
                <b-button
                  variant="outline-primary"
                  @click="importCoursesFromJSON"
                >
                  יבוא קורסים
                </b-button>
              </div>
            </b-modal>
          </b-dropdown-item>
        </b-nav-item-dropdown>
      </b-navbar-nav>
      <b-navbar-nav class="mr-auto">
        <b-navbar-brand
          href="#"
          mar
          style="padding-top: 5px; padding-bottom: 0; margin-left: 5px"
        >
          My Degree
        </b-navbar-brand>

        <img alt="" src="../assets/main_icon_white.svg" style="height: 36px" />
      </b-navbar-nav>
    </b-collapse>
  </b-navbar>
</template>

<script>
import firebase from "firebase/app";
import Authentication from "./HeaderAuthentication";
import "firebase/auth";
import "firebase/firestore";
import {
  parseCheeseFork,
  parseGraduateInformation,
  parseStudentsSiteGrades,
} from "@/store/extensions/converter";
import { createHelpers } from "vuex-map-fields";

const { mapFields } = createHelpers({
  getterType: "getUserField",
  mutationType: "updateUserField",
});

export default {
  name: "HeaderNavBar",
  components: { Authentication },
  data() {
    return {
      message: "",
      input_data: "",
      json_text: "",
      user_name: this.$store.state.user_name,
      logged: this.$store.state.logged,
      wrongInput: false,
    };
  },
  computed: {
    ...mapFields(["course_types"]),
  },
  mounted() {
    firebase.auth().onAuthStateChanged((user) => {
      if (user) {
        localStorage.setItem("authenticated", "true");
        this.logged = true;
        this.user = user;
        this.user_name = user.displayName;
        if (this.$refs["auth-modal"]) {
          this.$refs["auth-modal"].hide();
        }
        let uid = firebase.auth().currentUser.uid;
        firebase
          .firestore()
          .collection("users")
          .doc(uid)
          .get()
          .then((doc) => {
            if (doc.exists) {
              this.$store.commit("fetchUserInfo", doc.data());
              this.$store.commit("reCalcCurrentSemester");
            } else {
              firebase
                .firestore()
                .collection("users")
                .doc(uid)
                .set(this.$store.state.user)
                .catch((error) => {
                  // eslint-disable-next-line no-console
                  console.log("ErrorHeader - " + error.message);
                });
            }
            window.localStorage.removeItem("saved_session_data");
          })
          .catch((error) => {
            // eslint-disable-next-line no-console
            console.log("ErrorHeader2 - " + error.message);
          });
      }
    });
  },
  methods: {
    changeCategoryName(index, event) {
      this.$store.commit("changeCategoryName", [event.target.value, index]);
    },
    deleteCategory(index) {
      this.$bvModal
        .msgBoxConfirm("למחוק קטגוריה?", {
          title: "אזהרה",
          autoFocusButton: "ok",
          headerBgVariant: "dark",
          headerTextVariant: "white",
          size: "sm",
          buttonSize: "md",
          cancelDisabled: "true",
          okVariant: "danger",
          okTitle: "כן",
          cancelTitle: "לא",
          footerClass: "p-2",
          hideHeaderClose: true,
          centered: true,
        })
        .then((v) => {
          if (v === true) {
            this.$store.commit("deleteCourseType", index);
          }
        });
    },
    hideInvalidInput() {
      this.wrongInput = false;
      // document.getElementById('invalid-input').hidden = true
    },
    addCategory() {
      let new_category_name =
        document.getElementById("new_category_name").value;
      for (let course_type of this.course_types) {
        if (course_type.name === new_category_name) {
          this.wrongInput = true;
          return;
        }
      }
      this.$store.commit("addCourseType", new_category_name);
      this.hideModal("modal-add-course-type");
    },
    exportAsJson() {
      this.$store.commit("exportSemesters");
    },
    importCourseFromUG() {
      return this.importCoursesFromSite("UG");
    },
    importCourseFromStudents() {
      return this.importCoursesFromSite("Students");
    },
    importCoursesFromSite(site) {
      if (this.message !== "") {
        this.$bvModal
          .msgBoxConfirm("יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?", {
            title: "אזהרה",
            headerBgVariant: "dark",
            autoFocusButton: "ok",
            headerTextVariant: "white",
            size: "sm",
            buttonSize: "md",
            cancelDisabled: "true",
            okVariant: "danger",
            okTitle: "כן",
            cancelTitle: "לא",
            footerClass: "p-2",
            hideHeaderClose: true,
            centered: true,
          })
          .then((v) => {
            if (v === true) {
              let semesters_exemption_summerIndexes;
              if (site === "UG") {
                semesters_exemption_summerIndexes = parseGraduateInformation(
                  this.message
                );
              } else if (site === "Students") {
                semesters_exemption_summerIndexes = parseStudentsSiteGrades(
                  this.message
                );
              }
              this.$store.dispatch("loadUserDataFromSite", {
                semesters: semesters_exemption_summerIndexes["semesters"],
                exemption: semesters_exemption_summerIndexes["exemption"],
                summer_semesters_indexes:
                  semesters_exemption_summerIndexes["summer_semesters_indexes"],
              });
              this.message = "";
              if (site === "UG") {
                this.hideModal("modal-import-from-ug");
              } else if (site === "Students") {
                this.hideModal("modal-import-from-students");
              }
            }
          });
      }
    },
    importCoursesFromCF() {
      if (this.input_data !== "") {
        let courses_list = parseCheeseFork(this.input_data);
        this.$store.dispatch("addNewSemesterFromData", courses_list);
        this.input_data = "";
        this.hideModal("modal-cf-import");
      }
    },
    importCoursesFromJSON() {
      if (this.json_text !== "") {
        this.$bvModal
          .msgBoxConfirm("יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?", {
            title: "אזהרה",
            headerBgVariant: "dark",
            headerTextVariant: "white",
            size: "sm",
            buttonSize: "md",
            autoFocusButton: "ok",
            cancelDisabled: "true",
            okVariant: "danger",
            okTitle: "כן",
            cancelTitle: "לא",
            footerClass: "p-2",
            hideHeaderClose: true,
            centered: true,
          })
          .then((v) => {
            if (v === true) {
              this.$store.commit("importCoursesFromJson", this.json_text);
              this.$store.commit("reCalcCurrentSemester");
              this.json_text = "";
              this.hideModal("modal-import-from-json");
            }
          });
      }
    },
    signOut() {
      firebase.auth().signOut();
      localStorage.setItem("authenticated", "false");
      window.localStorage.removeItem("saved_session_data");
      this.logged = false;
      this.$store.commit("clearUserData");
    },
    hideModal(modalName) {
      if (this.$refs[modalName]) {
        this.$refs[modalName].hide();
      }
    },
  },
};
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
  min-width: 838px !important;
}

#modal-1 {
  min-width: 838px !important;
}

/*.navbar-custom {*/
/*  background-color: lightgreen;*/
/*}*/
/*!* Modify brand and text color *!*/

/*.navbar-custom .navbar-brand,*/
/*.navbar-custom .navbar-text {*/
/*  color: green;*/
/*}*/
</style>
