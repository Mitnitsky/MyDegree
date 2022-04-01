<template>
  <el-menu
    style="max-height: 60px !important"
    justify="end"
    mode="horizontal"
    background-color="#343a40"
    text-color="#fff"
    active-text-color="#ffd04b"
  >
    <template v-if="!logged">
      <el-menu-item index="1" @click="loggedInDialogVisible = true">
        <el-icon>
          <avatar />
        </el-icon>
        <span style="margin-right: 4px">כניסה</span></el-menu-item
      >
      <el-dialog
        v-model="loggedInDialogVisible"
        :modal="false"
        title="כניסה"
        center
      >
        <authentication :close_auth_modal="close_auth_modal" />
      </el-dialog>
    </template>
    <template v-else>
      <el-sub-menu index="1">
        <template #title>
          <el-icon>
            <avatar />
          </el-icon>
          <span style="margin-right: 4px; margin-left: 4px"
            >שלום {{ username }}</span
          >
        </template>
        <el-menu-item index="1-1">
          <el-icon>
            <right />
          </el-icon>
          <span style="margin-right: 4px">יציאה</span>
        </el-menu-item>
      </el-sub-menu>
    </template>
    <el-sub-menu index="2">
      <template #title>
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-left: 4px">יבוא</span>
      </template>
      <el-menu-item index="2-1" @click="studentsDialogVisible = true">
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-right: 4px"
          >יבוא קורסים מ-Students</span
        ></el-menu-item
      >
      <el-menu-item index="2-2" @click="gradesDialogVisible = true">
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-right: 4px">יבוא קורסים מ-UG</span></el-menu-item
      >

      <el-menu-item index="2-3" @click="cheeseforkDialogVisible = true">
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-right: 4px"
          >יבוא סמסטר מ-CheeseFork</span
        ></el-menu-item
      >

      <el-menu-item index="2-4" @click="jsonImportDialogVisible = true">
        <el-icon>
          <upload />
        </el-icon>
        <span style="margin-right: 4px">יבוא קורסים מקובץ-JSON</span>
      </el-menu-item>
    </el-sub-menu>
    <el-sub-menu index="3">
      <template #title>
        <el-icon>
          <upload />
        </el-icon>
        <span style="margin-left: 4px">יצוא</span>
      </template>
      <el-menu-item index="3-1" @click="exportAsJson(false)">
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-right: 4px"
          >יצוא קורסים לקובץ-JSON (ללא ציונים)</span
        >
      </el-menu-item>
      <el-menu-item index="3-2" @click="exportAsJson(true)">
        <el-icon>
          <download />
        </el-icon>
        <span style="margin-right: 4px"
          >יצוא קורסים לקובץ-JSON (עם ציונים)</span
        >
      </el-menu-item>
    </el-sub-menu>
    <el-menu-item index="4" @click="categoriesDialogVisible = true">
      <el-icon>
        <collection />
      </el-icon>
      <span style="margin-right: 4px">שינוי קטגוריות קורסים</span>
    </el-menu-item>
    <el-menu-item
      index="5"
      disabled
      style="
        position: absolute;
        left: 0;
        justify-content: end !important;
        opacity: 1 !important;
        cursor: context-menu !important;
      "
    >
      <span style="font-size: 20px">My Degree</span>
      <img
        alt=""
        src="../assets/main_icon_white.svg"
        style="height: 36px; margin-right: 5px"
      />
    </el-menu-item>
  </el-menu>

  <el-dialog
    v-model="gradesDialogVisible"
    title="Tips"
    width="30%"
    center
    :modal="false"
  >
    <template #title>
      <span class="dialog-title" style="font-weight: bold">
        יבוא קורסים וציונים מ-UG
      </span>
    </template>
    <el-row justify="center">
      <el-popover placement="top" :width="350" title="הוראות" trigger="click">
        <template #reference>
          <el-button type="info" plain>הוראות </el-button>
        </template>
        <template #default>
          <p class="popover-body-text">
            יש לסמן את כל התוכן באמצעות CTRL+A
            <br />
            <a
              href="https://techmvs.technion.ac.il/cics/wmn/wmngrad?ORD=1"
              target="_blank"
              >באתר ציונים</a
            >
            ולהעתיק אותו לתיבת הטקסט בחלון זה
            <br />
            (<b>אפשרי להעתיק רק את הסמסטרים</b>)
          </p>
        </template>
      </el-popover>
    </el-row>
    <el-row justify="center">
      <el-input
        class="input-grades-box"
        v-model="message"
        :rows="5"
        resize="none"
        type="textarea"
        placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
      />
    </el-row>
    <el-row justify="center">
      <el-button type="primary" @click="importCourseFromUG"
        >יבוא קורסים
      </el-button>
    </el-row>
  </el-dialog>

  <el-dialog width="30%" v-model="studentsDialogVisible" :modal="false" center>
    <template #title>
      <span class="dialog-title" style="font-weight: bold">
        יבוא קורסים מ-Students
      </span>
    </template>
    <el-row justify="center">
      <el-popover placement="top" :width="350" title="הוראות" trigger="click">
        <template #reference>
          <el-button type="info" plain>הוראות </el-button>
        </template>
        <template #default>
          <p class="popover-body-text">
            יש לסמן את כל התוכן באמצעות CTRL+A
            <br />
            <a
              href="https://students.technion.ac.il/local/tcurricular/grades"
              target="_blank"
              >באתר ציונים</a
            >
            ולהעתיק אותו לתיבת הטקסט בחלון זה
            <br />
            (<b>אפשרי להעתיק רק את הסמסטרים</b>)
          </p>
        </template>
      </el-popover>
    </el-row>
    <el-row justify="center">
      <el-input
        class="input-grades-box"
        v-model="message"
        :rows="5"
        resize="none"
        type="textarea"
        placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
      />
    </el-row>
    <el-row justify="center">
      <el-button type="primary" @click="importCourseFromStudents"
        >יבוא קורסים
      </el-button>
    </el-row>
  </el-dialog>

  <el-dialog
    width="30%"
    v-model="cheeseforkDialogVisible"
    :modal="false"
    center
  >
    <template #title>
      <span class="dialog-title" style="font-weight: bold">
        יבוא סמסטר מ-CheeseFork
      </span>
    </template>
    <el-row justify="center">
      <el-popover placement="left" :width="350" title="הוראות" trigger="click">
        <template #reference>
          <el-button type="info" plain>הוראות </el-button>
        </template>
        <template #default>
          <p class="popover-body-text">
            יש לסמן את הורסים באתר
            <a href="https://cheesefork.cf/" target="_blank">CheeseFork</a>
            ולהעתיק אותו<br />לתיבת הטקסט בחלון זה
          </p>
          <el-row style="justify-content: center"
            ><img src="../../images/import_from_cf.png"
          /></el-row>
        </template>
      </el-popover>
    </el-row>
    <el-row justify="center">
      <el-input
        class="input-grades-box"
        v-model="message"
        :rows="5"
        resize="none"
        type="textarea"
        placeholder="יש להעתיק את התוכן מאתר הציונים לכאן"
      />
    </el-row>
    <el-row justify="center">
      <el-button type="primary" @click="importCoursesFromCF"
        >יבוא קורסים
      </el-button>
    </el-row>
  </el-dialog>

  <el-dialog
    width="30%"
    v-model="jsonImportDialogVisible"
    :modal="false"
    center
  >
    <template #title>
      <span class="dialog-title" style="font-weight: bold">
        יבוא נתונים מקובץ JSON
      </span>
    </template>
    <el-row justify="center">
      <el-input
        class="input-grades-box"
        v-model="message"
        :rows="5"
        resize="none"
        type="textarea"
        placeholder="יש להעתיק את התוכן קובץ הJSON"
      />
    </el-row>
    <el-row justify="center">
      <el-button type="primary" @click="importCoursesFromJSON"
        >יבוא קורסים
      </el-button>
    </el-row>
  </el-dialog>

  <el-dialog
    width="40%"
    v-model="categoriesDialogVisible"
    :modal="false"
    center
  >
    <template #title>
      <span class="dialog-title" style="font-weight: bold">
        שינוי קטגוריות קורסים
      </span>
    </template>
    <el-row justify="center">
      <el-table :data="course_types">
        <el-table-column align="right">
          <template #header>
            <h4
              style="
                color: white;
                text-align: center;
                background-color: #343a40;
                margin-bottom: 0;
                margin-top: 0;
              "
            >
              קטגוריות
            </h4>
          </template>
          <template #default="scope">
            <template v-if="scope.$index === 0 || scope.$index === 1">
              <el-input disabled v-model="scope.row.name" />
            </template>
            <template v-else>
              <el-row :gutter="2">
                <el-col :span="21" s>
                  <el-input v-model="scope.row.name" />
                </el-col>
                <el-col :span="1">
                  <el-button
                    style="margin-top: 4px"
                    type="danger"
                    plain
                    size="small"
                    @click="deleteCategory(scope.$index)"
                    >x
                  </el-button>
                </el-col>
              </el-row>
            </template>
          </template>
        </el-table-column>
      </el-table>
    </el-row>
    <el-row justify="center" style="margin-top: 10px">
      <el-button type="primary" @click="addCategory">הוסף קטגוריה </el-button>
    </el-row>
  </el-dialog>
</template>

<script lang="ts">
import { defineComponent, computed, WritableComputedRef, ref } from "vue";
import Authentication from "@/components/HeaderAuthentication.vue";
import {
  parseCheeseFork,
  parseGraduateInformation,
  parseStudentsSiteGrades,
} from "@/store/extensions/converter";

import { db, auth } from "@/main";
import { useStore } from "@/use/useStore";
import { AUTH_STORE, USER_STORE } from "@/store/constants";
import ElDialog from "element-plus/es/components/dialog";
import { ElMessage, ElMessageBox } from "element-plus";
import {
  Avatar,
  Collection,
  Download,
  Right,
  Upload,
} from "@element-plus/icons-vue";
import { stateConverter } from "@/firestore/firestoreconverter";
import { CourseType } from "@/store/classes/course_types";

export default defineComponent({
  name: "HeaderNavBar",
  components: {
    Upload,
    Collection,
    Download,
    Right,
    Avatar,
    Authentication,
  },
  setup() {
    enum HeaderModal {
      UG = "UG",
      STUDENTS = "STUDENTS",
      CHEESEFORK = "CHEESEFORK",
      CATEGORY_CHANGE = "CATEGORY_CHANGE",
      JSON_IMPORT_GRADES = "JSON_IMPORT_GRADES",
    }

    const message = ref("");
    const input_data = ref("");
    const json_text = ref("");
    const wrongInput = ref(false);
    const loggedInDialogVisible = ref(false);
    const gradesDialogVisible = ref(false);
    const studentsDialogVisible = ref(false);
    const cheeseforkDialogVisible = ref(false);
    const categoriesDialogVisible = ref(false);
    const jsonImportDialogVisible = ref(false);
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
    const course_types = computed<CourseType[]>(() => {
      return store.getters[USER_STORE.GETTERS.COURSE_TYPES];
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
      ElMessageBox.confirm("למחוק קטגוריה?", {
        confirmButtonText: "כן",
        cancelButtonText: "לא",
        icon: "none",
        type: "warning",
      })
        .then(() => {
          store.commit(USER_STORE.MUTATIONS.deleteCourseType, index);
        })
        .catch(() => {
          // catch error
        });
    };
    const hideInvalidInput = () => {
      wrongInput.value = false;
    };
    const signOut = () => {
      auth.signOut();
      localStorage.setItem("authenticated", "false");
      window.localStorage.removeItem("saved_session_data");
      logged.value = false;
      store.commit(USER_STORE.MUTATIONS.clearUserData);
    };
    const exportAsJson = (with_grades) => {
      store.commit(USER_STORE.MUTATIONS.exportSemesters, with_grades);
    };
    const addCategory = () => {
      ElMessageBox.prompt("הכנס שם קטגוריה:", "הוספת קטגוריה", {
        confirmButtonText: "הוסף",
        icon: "none",
      }).then(({ value }) => {
        let types = store.getters[USER_STORE.GETTERS.COURSE_TYPES];
        for (let i = 0; i < types.length; i++) {
          if (types[i].name === value) {
            wrongInput.value = true;
            ElMessage({
              type: "info",
              message: `קטוגריה עם שם כזה קיימת כבר!`,
            });
            return;
          }
        }
        store.commit(USER_STORE.MUTATIONS.addCourseType, value);
        hideModal(HeaderModal.CATEGORY_CHANGE);
        ElMessage({
          type: "success",
          message: `קטוגריה הוספה בהצלחה!`,
        });
      });
    };
    const importCourseFromUG = () => {
      return importCoursesFromSite("UG");
    };
    const importCourseFromStudents = () => {
      return importCoursesFromSite("Students");
    };
    const importCoursesFromSite = (site) => {
      if (message.value !== "") {
        ElMessageBox.confirm(
          "יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?",
          "אזהרה",
          {
            confirmButtonText: "כן",
            cancelButtonText: "לא",
            type: "warning",
            icon: "none",
          }
        )
          .then(() => {
            let semesters_exemption_summerIndexes;
            if (site === "UG") {
              semesters_exemption_summerIndexes = parseGraduateInformation(
                message.value
              );
            } else if (site === "Students") {
              semesters_exemption_summerIndexes = parseStudentsSiteGrades(
                message.value
              );
            }
            store.dispatch(USER_STORE.ACTIONS.loadUserDataFromSite, {
              semesters: semesters_exemption_summerIndexes["semesters"],
              exemption: semesters_exemption_summerIndexes["exemption"],
              summer_semesters_indexes:
                semesters_exemption_summerIndexes["summer_semesters_indexes"],
            });
            message.value = "";
            if (site === "UG") {
              hideModal(HeaderModal.UG);
            } else if (site === "Students") {
              hideModal(HeaderModal.STUDENTS);
            }
            ElMessage({
              type: "success",
              message: "יבוא הושלם",
            });
          })
          .catch(() => {
            ElMessage({
              type: "info",
              message: "יבוא נכשל",
            });
          });
      }
    };
    const importCoursesFromCF = () => {
      if (input_data.value !== "") {
        let courses_list = parseCheeseFork(input_data.value);
        store.dispatch(USER_STORE.ACTIONS.addNewSemesterFromData, courses_list);
        input_data.value = "";
        hideModal(HeaderModal.CHEESEFORK);
      }
    };
    const importCoursesFromJSON = () => {
      if (json_text.value !== "") {
        ElMessageBox.confirm("יבוא קורסים ימחק כל תוכן הקיים באתר, להמשיך?", {
          confirmButtonText: "כן",
          cancelButtonText: "לא",
          type: "warning",
          icon: "none",
        })
          .then(() => {
            store.commit(
              USER_STORE.MUTATIONS.importCoursesFromJson,
              json_text.value
            );
            store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
            json_text.value = "";
            hideModal(HeaderModal.JSON_IMPORT_GRADES);
            ElMessage({
              type: "success",
              message: "יבוא הושלם",
            });
          })
          .catch(() => {
            ElMessage({
              type: "info",
              message: "יבוא נכשל",
            });
          });
      }
    };
    const hideModal = (modal: HeaderModal) => {
      if (modal == HeaderModal.UG) {
        gradesDialogVisible.value = false;
      } else if (modal == HeaderModal.CATEGORY_CHANGE) {
        gradesDialogVisible.value = false;
      } else if (modal == HeaderModal.CHEESEFORK) {
        cheeseforkDialogVisible.value = false;
      } else if (modal == HeaderModal.JSON_IMPORT_GRADES) {
        jsonImportDialogVisible.value = false;
      } else if (modal == HeaderModal.STUDENTS) {
        studentsDialogVisible.value = false;
      } else {
        console.log("Wrong header modal passed!");
      }
    };
    return {
      course_types,
      message,
      input_data,
      json_text,
      wrongInput,
      loggedInDialogVisible,
      gradesDialogVisible,
      studentsDialogVisible,
      cheeseforkDialogVisible,
      categoriesDialogVisible,
      jsonImportDialogVisible,
      username,
      logged,
      close_auth_modal,
      handleClose,
      changeCategoryName,
      deleteCategory,
      hideInvalidInput,
      signOut,
      exportAsJson,
      addCategory,
      importCourseFromUG,
      importCourseFromStudents,
      importCoursesFromCF,
      importCoursesFromJSON,
    };
  },
  mounted() {
    const store = useStore();
    auth.onAuthStateChanged((user) => {
      if (user) {
        this.loggedInDialogVisible = false;
        localStorage.setItem("authenticated", "true");
        this.logged = true;
        this.username = user.displayName ? user.displayName : "";
        if (this.$refs["auth-modal"]) {
          (this.$refs["auth-modal"] as typeof ElDialog).close();
        }
        let uid = user.uid;
        db.collection("users")
          .withConverter(stateConverter)
          .doc(uid)
          .get()
          .then((doc) => {
            if (doc.exists) {
              store.commit(USER_STORE.MUTATIONS.fetchUserInfo, doc.data());
              store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
            } else {
              db.collection("users")
                .withConverter(stateConverter)
                .doc(uid)
                .set(store.state.user.state)
                .then((result) => {
                  result;
                })
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
});
</script>

<style>
@import "../fonts/Alef/stylesheet.css";

.el-sub-menu__title {
  font-size: 16px !important;
}

.el-menu-item {
  font-size: 16px !important;
}

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

div.el-dialog__header {
  padding-bottom: 20px;
  border-top-right-radius: 5px;
  border-top-left-radius: 5px;
  color: white;
  background-color: #343a40 !important;
}

.el-dialog {
  direction: rtl;
  border-bottom-right-radius: 5px;
  border-bottom-left-radius: 5px;
}

.el-popper {
  font-family: Alef, Roboto, Helvetica, Arial, sans-serif !important;
  direction: rtl;
  padding-left: 0;
  box-shadow: var(--el-dialog-box-shadow);
  padding-right: 0;
}

div.el-popper.el-popover {
  padding: 0;
  border: 1px solid;
  border-color: #e6e6e6;
  border-radius: 5px;
}

.el-popover__title {
  font-size: 0.83em;
  padding: 12px;
  font-weight: bold;
  background-color: #f7f7f7;
  border-top-right-radius: 5px;
  border-top-left-radius: 5px;
  direction: rtl;
}

div.el-message-box__title {
  text-align: center;
}

.el-dialog__headerbtn {
  right: auto !important;
  left: var(--el-dialog-padding-primary);
}

.el-message-box__message {
  width: auto;
  text-align: start;
  padding-left: 0 !important;
  padding-right: 12px !important;
}

.el-message-box__status {
  transform: translate(700%, -220%) !important;
}

.el-message-box__headerbtn {
  right: auto !important;
  left: var(--el-messagebox-padding-primary);
}

.input-grades-box {
  padding-top: 1em;
  padding-bottom: 1em;
}

.popover-body-text {
  padding: 0.5rem 0.75rem;
}

.el-message-box {
  font-family: Alef, Roboto, Helvetica, Arial, sans-serif !important;
  direction: rtl !important;
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
