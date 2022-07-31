<template>
  <el-card style="justify-content: center">
    <el-tabs
      addable
      class="demo-tabs"
      dir="rtl"
      justify="end"
      style="font-size: 16px !important"
      type="card"
      @edit="handleSemesterEdit"
      @tab-click="changeActiveSemester"
    >
      <el-tab-pane
        v-for="(semester, index) in semesters"
        :key="index"
        :label="'סמסטר ' + semester.name"
        :name="index"
        dir="rtl"
        justify="end"
      >
        <template #label>
          <span style="margin-left: 4px">
            {{ "סמסטר " + semester.name }}
          </span>
          <font-awesome-icon
            v-if="semester.name.includes('קיץ')"
            icon="sun"
            style="color: orange"
          />
        </template>
        <el-row justify="center">
          <el-col :span="20">
            <el-row justify="center">
              <app-semester-table :semester="semester" />
            </el-row>
            <el-row justify="center">
              <el-button-group style="padding: 8px">
                <el-button
                  class="semester-tab-button"
                  color="#17a2b8"
                  style="color: white"
                  >הוספת שורה
                </el-button>
                <el-button class="semester-tab-button" type="primary"
                  >חיפוש קורסים
                </el-button>
              </el-button-group>
            </el-row>
          </el-col>
          <el-col
            :span="4"
            style="
              min-width: 224px !important;
              max-width: 300px;
              padding-right: 24px;
            "
          >
            <el-card shadow="never">
              <template #header>
                <span class="card-header-text">סיכום סמסטר</span>
              </template>
              <div style="justify-content: center; padding: 1rem 1.5rem">
                <el-row>
                  <el-col :span="10">
                    <span style="margin-left: 4px">ממוצע:</span>
                  </el-col>
                  <el-col :span="14">
                    <el-input
                      :model-value="semester.average"
                      class="semester-info-text"
                      disabled
                    />
                  </el-col>
                </el-row>
                <el-row style="margin-top: 0.5rem">
                  <el-col :span="10">
                    <span style="margin-left: 4px">נקודות:</span>
                  </el-col>
                  <el-col :span="14">
                    <el-input
                      :model-value="semester.points"
                      class="semester-info-text"
                      disabled
                    />
                  </el-col>
                </el-row>
              </div>
            </el-card>
          </el-col>
        </el-row>
        <el-row justify="end" style="margin-top: 10px">
          <el-button
            class="align-self-end"
            color="#17A2B8"
            plain
            style="margin-left: 4px"
            variant="primary"
            @click="changeSemesterType(semester.name)"
          >
            הפוך לסמסטר {{ semester.name.includes("קיץ") ? "רגיל" : "קיץ" }}
          </el-button>
          <el-button
            class="align-self-end"
            plain
            type="danger"
            @click="removeSemester"
          >
            מחק סמסטר
          </el-button>
        </el-row>
      </el-tab-pane>
    </el-tabs>
  </el-card>
</template>

<script lang="ts">
import { computed, defineComponent } from "vue";
// import AppSemesterSummary from "@/components/SemesterSummary.vue";
import AppSemesterTable from "@/components/SemesterTable.vue";
import firebase from "firebase/compat/app";
import "firebase/compat/auth";
import "firebase/compat/firestore";
import { ElMessage, ElMessageBox } from "element-plus/es";
import { USER_STORE } from "@/store/constants";
import { useStore } from "@/use/useStore";
import { Semester } from "@/store/classes/semester";

export default defineComponent({
  name: "SemestersTabView",
  components: { AppSemesterTable },
  setup() {
    const store = useStore();
    const activeSemester = computed({
      get(): string {
        return (
          +store.getters[USER_STORE.GETTERS.ACTIVE_SEMESTER] + 1
        ).toString();
      },
      set(value: string) {
        console.log(value);
        store.commit(USER_STORE.MUTATIONS.setActiveSemester, +value - 1);
      },
    });
    const removeSemester = () => {
      ElMessageBox.confirm("למחוק סמסטר זה?", {
        confirmButtonText: "כן",
        cancelButtonText: "לא",
        type: "warning",
        icon: "none",
      })
        .then(() => {
          store.commit(USER_STORE.MUTATIONS.removeSemester);
          store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
          ElMessage({
            type: "success",
            message: "סמסטר נמחק בהצלחה",
          });
        })
        .catch(() => {
          ElMessage({
            type: "info",
            message: "המחיקה בוטלה",
          });
        });
    };
    const semesters = computed<Semester[]>(() => {
      return store.getters[USER_STORE.GETTERS.SEMESTERS];
    });
    const newTab = () => {
      store.commit(USER_STORE.MUTATIONS.addSemester, 1);
    };
    const changeSemesterType = (semester_name) => {
      if (semester_name.includes("קיץ")) {
        changeToRegular();
      } else {
        changeToSummer();
      }
    };
    const changeToSummer = () => {
      store.commit(USER_STORE.MUTATIONS.changeActiveSemesterType);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const changeToRegular = () => {
      store.commit(USER_STORE.MUTATIONS.changeActiveSemesterType);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const handleSemesterEdit = (
      targetName: string,
      action: "remove" | "add"
    ) => {
      if (action === "add") {
        store.commit(USER_STORE.MUTATIONS.addSemester, 1);
      } else if (action === "remove") {
        removeSemester();
      }
    };
    const changeActiveSemester = (element) => {
      store.commit(USER_STORE.MUTATIONS.setActiveSemester, element.props.name);
    };
    return {
      changeActiveSemester,
      handleSemesterEdit,
      activeSemester,
      semesters,
      removeSemester,
      newTab,
      changeSemesterType,
    };
  },
  mounted() {
    const store = useStore();
    const authentication_status = localStorage.getItem("authenticated");
    const user = firebase.auth().currentUser;
    if (user == null) {
      if (authentication_status === "false") {
        const user_data = localStorage.getItem("saved_session_data");
        if (user_data == null) {
          return;
        }
        if (typeof user_data === "object") {
          store.commit(USER_STORE.MUTATIONS.fetchUserInfo, user_data);
        } else {
          const session_data = localStorage.getItem("saved_session_data");
          if (session_data != null) {
            store.commit(
              USER_STORE.MUTATIONS.fetchUserInfo,
              JSON.parse(session_data)
            );
          }
        }
      }
    }
  },
});
</script>
<style>
div.el-tabs__header {
  display: flex;
  flex-direction: row-reverse;
  justify-content: flex-end;
}

.el-tabs--card > .el-tabs__header .el-tabs__item:first-child {
  border-left: 1px solid #e5e4ed !important;
  border-right: 0 !important;
  border-top-right-radius: 5px !important;
}

.el-tabs--card > .el-tabs__header .el-tabs__item:last-child {
  border-right: unset;
  border-top-left-radius: 5px !important;
  border-left: 0 !important;
}

.el-tabs--card > .el-tabs__header .el-tabs__item.is-active {
  border-bottom-color: #409eff !important;
  border-bottom: 1px solid !important;
}

.el-tabs--card > .el-tabs__header {
  border-bottom: 0 solid var(--el-border-color-light) !important;
}

.el-tabs--card > .el-tabs__header .el-tabs__item {
  border-bottom: 1px solid #e5e4ed !important;
  transition: color var(--el-transition-duration)
      var(--el-transition-function-ease-in-out-bezier),
    padding var(--el-transition-duration)
      var(--el-transition-function-ease-in-out-bezier);
}

span.el-tabs__new-tab {
  background: white !important;
}

.is-icon-plus {
  color: black !important;
}

.is-icon-plus:hover {
  color: cornflowerblue !important;
}

.el-tabs__header {
  padding: 0.75rem 1.25rem !important;
  margin: 0 !important;
  background-color: rgba(0, 0, 0, 0.03) !important;
  border-bottom: 1px solid rgba(0, 0, 0, 0.125) !important;
  border-radius: calc(0.25rem - 1px) calc(0.25rem - 1px) 0 0 !important;
}

.el-tabs__content {
  padding: 0.75rem 1.25rem !important;
  border-top: 1px solid rgba(0, 0, 0, 0.125) !important;
  border-bottom-left-radius: calc(0.25rem - 1px) calc(0.25rem - 1px);
  border-bottom-right-radius: calc(0.25rem - 1px) calc(0.25rem - 1px);
}

.el-card__body {
  padding: 0 !important;
}

th > div.cell {
  color: #495057;
  font-weight: bold;
  text-align: center;
  font-size: 16px !important;
}

div.el-tabs__item {
  background: white !important;
}

.el-card__header {
  background-color: rgb(233, 236, 239) !important;
  padding-bottom: 3px !important;
  padding-top: 3px !important;
  text-align: center;
  border-bottom: 1px solid rgba(0, 0, 0, 0.125);
}

.card-header-text {
  color: #495057;
  font-weight: bold;
  text-align: center;
  font-size: 16px !important;
}

th.is-sortable > div:hover {
  color: cornflowerblue !important;
  text-decoration: underline !important;
  cursor: pointer !important;
}

.clickAbleHeader:hover {
  color: cornflowerblue;
  text-decoration: underline;
  cursor: pointer;
}

div.semester-info-text > input {
  text-align: center !important;
  background: white !important;
  color: black !important;
  cursor: default !important;
}

.semester-tab-button {
  font-size: 16px !important;
  font-family: Alef, Roboto, Helvetica, Arial, sans-serif !important;
}

div.el-tabs__item {
  font-size: 16px !important;
  font-family: Alef, Roboto, Helvetica, Arial, sans-serif !important;
}
</style>
