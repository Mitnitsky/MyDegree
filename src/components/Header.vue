<template>
  <el-menu
    justify="end"
    mode="horizontal"
    background-color="#545c64"
    text-color="#fff"
    active-text-color="#ffd04b"
  >
    <template v-if="!logged">
      <el-menu-item index="1" @click="loggedInDialogVisible = true">
        <el-icon><avatar /></el-icon>
        <span style="margin-right: 4px">כניסה</span></el-menu-item
      >
      <el-dialog
        id="auth-modal"
        v-model="loggedInDialogVisible"
        title="כניסה"
        center
      >
        <authentication :close_auth_modal="close_auth_modal" />
      </el-dialog>
    </template>
    <template v-else>
      <el-sub-menu index="1">
        <template #title>
          <el-icon><avatar /></el-icon>
          <span style="margin-right: 4px; margin-left: 4px"
            >שלום {{ username }}</span
          >
        </template>
        <el-menu-item index="1-1" style="direction: rtl !important">
          <el-icon><right /></el-icon>
          <span style="margin-right: 4px">יציאה</span>
        </el-menu-item>
      </el-sub-menu>
    </template>

    <el-menu-item index="2" @click="gradesDialogVisible = true">
      <el-icon><download /></el-icon>
      <span style="margin-right: 4px">יבוא קורסים מ-UG</span></el-menu-item
    >
    <el-dialog
      id="grades-modal"
      v-model="gradesDialogVisible"
      title="יבוא קורסים מ-UG"
      center
    >
    </el-dialog>

    <el-menu-item index="3" @click="studentsDialogVisible = true">
      <el-icon><download /></el-icon>
      <span style="margin-right: 4px"
        >יבוא קורסים מ-Students</span
      ></el-menu-item
    >
    <el-dialog
      id="students-modal"
      v-model="studentsDialogVisible"
      title="יבוא קורסים מ-Students"
      center
    >
    </el-dialog>

    <el-menu-item index="4" @click="cheeseforkDialogVisible = true">
      <el-icon><download /></el-icon>
      <span style="margin-right: 4px"
        >יבוא סמסטר מ-CheeseFork</span
      ></el-menu-item
    >
    <el-dialog
      id="cheesefork-modal"
      v-model="cheeseforkDialogVisible"
      title="יבוא סמסטר מ-CheeseFork"
      center
    >
    </el-dialog>

    <el-menu-item index="5" @click="categoriesDialogVisible = true">
      <el-icon><collection /></el-icon>
      <span style="margin-right: 4px">שינוי קטגוריות קורסים</span></el-menu-item
    >
    <el-dialog
      id="categories-modal"
      v-model="categoriesDialogVisible"
      title="שינוי קטגוריות קורסים"
      center
    >
    </el-dialog>

    <el-sub-menu index="6">
      <template #title>
        <el-icon><expand /></el-icon>
      </template>
      <el-menu-item index="6-1" style="direction: rtl !important">
        <el-icon><download /></el-icon>
        <span style="margin-right: 4px"
          >יצוא קורסים לקובץ-JSON (ללא ציונים)</span
        >
      </el-menu-item>
      <el-menu-item index="6-2" style="direction: rtl !important">
        <el-icon><download /></el-icon>
        <span style="margin-right: 4px"
          >יצוא קורסים לקובץ-JSON (עם ציונים)</span
        >
      </el-menu-item>
      <el-menu-item index="6-3" style="direction: rtl !important">
        <el-icon><upload /></el-icon>
        <span style="margin-right: 4px">יבוא קורסים מקובץ-JSON</span>
      </el-menu-item>
    </el-sub-menu>

    <el-menu-item index="7" style="direction: ltr !important; width: 100%">
      <el-row style="width: 100% !important">
        <img
          alt=""
          src="../assets/main_icon_white.svg"
          style="height: 36px; margin-top: 10px"
        />
        <span style="margin-left: 8px">My Degree</span>
      </el-row>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts">
import { defineComponent, computed, WritableComputedRef, ref } from "vue";

import Authentication from "@/components/HeaderAuthentication.vue";
import firebase from "firebase/compat/app";
import "firebase/compat/auth";
import "firebase/compat/firestore";
import {
  parseCheeseFork,
  parseGraduateInformation,
  parseStudentsSiteGrades,
} from "@/store/extensions/converter";

import { useStore } from "@/use/useStore";
import { AUTH_STORE, USER_STORE } from "@/store/constants";
import ElDialog from "element-plus/es/components/dialog";
import { ElMessage, ElMessageBox } from "element-plus";
import {
  Avatar,
  Collection,
  Download,
  Expand,
  Right,
  Upload,
} from "@element-plus/icons-vue";

export default defineComponent({
  name: "HeaderNavBar",
  components: {
    Upload,
    Expand,
    Collection,
    Download,
    Right,
    Avatar,
    Authentication,
  },
  setup() {
    const message = ref("");
    const input_data = ref("");
    const json_text = ref("");
    const wrongInput = ref(false);
    const new_category_name = ref("");
    const students_site_dialog_visible = ref(false);
    const ug_site_dialog_visible = ref(false);
    const loggedInDialogVisible = ref(false);
    const gradesDialogVisible = ref(false);
    const studentsDialogVisible = ref(false);
    const cheeseforkDialogVisible = ref(false);
    const categoriesDialogVisible = ref(false);
    const store = useStore();
    const close_auth_modal = () => {
      loggedInDialogVisible.value = false;
    };
    const username: WritableComputedRef<string> = computed({
      get(): string {
        return store.getters[AUTH_STORE.GETTERS.USERNAME];
      },
      set(username: string): void {
        store.commit(AUTH_STORE.MUTATIONS.setUserName, username);
      },
    });
    const logged: WritableComputedRef<boolean> = computed({
      get(): boolean {
        return store.getters[AUTH_STORE.GETTERS.LOGGED];
      },
      set(logged: boolean): void {
        store.commit(AUTH_STORE.MUTATIONS.setLoggedStatus, logged);
      },
    });
    const handleClose = (done: () => void) => {
      ElMessageBox.confirm("Are you sure to close this dialog?")
        .then(() => {
          done();
        })
        .catch(() => {
          // catch error
        });
    };
    const changeCategoryName = (index, event) => {
      store.commit(USER_STORE.MUTATIONS.changeCategoryName, [
        event.target.value,
        index,
      ]);
    };
    const deleteCategory = (index) => {
      store.commit(USER_STORE.MUTATIONS.deleteCourseType, index);
    };
    const hideInvalidInput = () => {
      wrongInput.value = false;
    };
    const signOut = () => {
      firebase.auth().signOut();
      localStorage.setItem("authenticated", "false");
      window.localStorage.removeItem("saved_session_data");
      logged.value = false;
      store.commit(USER_STORE.MUTATIONS.clearUserData);
    };
    const exportAsJson = (with_grades) => {
      store.commit(USER_STORE.MUTATIONS.exportSemesters, with_grades);
    };

    return {
      message,
      input_data,
      json_text,
      wrongInput,
      new_category_name,
      students_site_dialog_visible,
      ug_site_dialog_visible,
      loggedInDialogVisible,
      gradesDialogVisible,
      studentsDialogVisible,
      cheeseforkDialogVisible,
      categoriesDialogVisible,
      username,
      logged,
      close_auth_modal,
    };
  },
  mounted() {
    const store = useStore();
    firebase.auth().onAuthStateChanged((user) => {
      if (user) {
        this.loggedInDialogVisible = false;
        localStorage.setItem("authenticated", "true");
        this.logged = true;
        this.username = user.displayName ? user.displayName : "";
        if (this.$refs["auth-modal"]) {
          (this.$refs["auth-modal"] as typeof ElDialog).close();
        }
        let uid = user.uid;
        firebase
          .firestore()
          .collection("users")
          .doc(uid)
          .get()
          .then((doc) => {
            if (doc.exists) {
              store.commit(USER_STORE.MUTATIONS.fetchUserInfo, doc.data());
              store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
            } else {
              firebase
                .firestore()
                .collection("users")
                .doc(uid)
                .set(store.state.user.state)
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
    const addCategory = () => {
      for (let course_type of store.state.user.state.course_types) {
        if (course_type.name === this.new_category_name) {
          this.wrongInput = true;
          return;
        }
      }
      store.commit(USER_STORE.MUTATIONS.addCourseType, this.new_category_name);
      hideModal("modal-add-course-type");
    };
    const importCourseFromUG = () => {
      return importCoursesFromSite("UG");
    };
    const importCourseFromStudents = () => {
      return importCoursesFromSite("Students");
    };
    const importCoursesFromSite = (site) => {
      if (this.message !== "") {
        ElMessageBox.confirm(
          "יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?",
          "אזהרה",
          {
            confirmButtonText: "כן",
            cancelButtonText: "לא",
            type: "warning",
          }
        )
          .then(() => {
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
            store.dispatch(USER_STORE.ACTIONS.loadUserDataFromSite, {
              semesters: semesters_exemption_summerIndexes["semesters"],
              exemption: semesters_exemption_summerIndexes["exemption"],
              summer_semesters_indexes:
                semesters_exemption_summerIndexes["summer_semesters_indexes"],
            });
            this.message = "";
            if (site === "UG") {
              hideModal("modal-import-from-ug");
            } else if (site === "Students") {
              hideModal("modal-import-from-students");
            }
            ElMessage({
              type: "success",
              message: "יבוא הושלם",
            });
          })
          .catch(() => {
            ElMessage({
              type: "info",
              message: "יבוא בוטל",
            });
          });
      }
    };
    const importCoursesFromCF = () => {
      if (this.input_data !== "") {
        let courses_list = parseCheeseFork(this.input_data);
        store.dispatch(USER_STORE.ACTIONS.addNewSemesterFromData, courses_list);
        this.input_data = "";
        hideModal("modal-cf-import");
      }
    };
    const importCoursesFromJSON = () => {
      if (this.json_text !== "") {
        ElMessageBox.confirm("יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?", {
          confirmButtonText: "כן",
          cancelButtonText: "לא",
          type: "warning",
        })
          .then(() => {
            store.commit(
              USER_STORE.MUTATIONS.importCoursesFromJson,
              this.json_text
            );
            store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
            this.json_text = "";
            hideModal("modal-import-from-json");
            ElMessage({
              type: "success",
              message: "יבוא הושלם",
            });
          })
          .catch(() => {
            ElMessage({
              type: "info",
              message: "יבוא בוטל",
            });
          });
      }
    };
    const hideModal = (modalName) => {
      if (this.$refs[modalName]) {
        (this.$refs[modalName] as typeof ElDialog).close();
      }
    };
  },
});
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
