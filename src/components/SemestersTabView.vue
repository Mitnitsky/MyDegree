<template>
  <el-card>
    <el-tabs
      dir="rtl"
      justify="end"
      v-model="editableTabsValue"
      type="card"
      editable
      class="demo-tabs"
      @edit="handleTabsEdit"
    >
      <el-tab-pane
        justify="end"
        dir="rtl"
        v-for="(semester, index) in semesters"
        :key="index"
        :label="'סמסטר ' + semester.name"
        :name="semester.name"
      >
        <app-semester-table :semester="semester" />
        <el-button
          v-if="semester.name.toString().includes('קיץ')"
          class="align-self-end"
          size="sm"
          variant="outline-info"
          @click="changeToRegular"
        >
          הפוך לסמסטר רגיל
        </el-button>
        <el-button
          v-else
          class="align-self-end"
          size="sm"
          variant="outline-info"
          @click="changeToSummer"
        >
          הפוך לסמסטר קיץ
        </el-button>
      </el-tab-pane>
    </el-tabs>
  </el-card>
  <!--  <b-card class="shadow bg-white rounded"-->
  <!--          no-body-->
  <!--          style="margin: 10px 20px">-->
  <!--    <b-tabs pills-->
  <!--            card-->
  <!--            v-model="active_semester">-->
  <!--      <b-tab-->
  <!--        v-for="(semester, index) in $store.state.user.semesters"-->
  <!--        :key="index"-->
  <!--        :title="'סמסטר ' + semester.name"-->
  <!--        lazy-->
  <!--      >-->
  <!--        <div class="row justify-content-md-center">-->
  <!--          <div class="col-xl-10"-->
  <!--               style="margin-bottom: 10px">-->
  <!--            <app-semester-table :semester="semester" />-->
  <!--          </div>-->
  <!--          <div class="col-xl-2"-->
  <!--               style="padding: 0 0">-->
  <!--            <app-semester-summary />-->
  <!--          </div>-->
  <!--        </div>-->
  <!--        <div class="row">-->
  <!--          <div class="col-xl-10" />-->
  <!--          <div class="col-xl-2">-->
  <!--            <b-button-group class="mx-1 mt-2"-->
  <!--                            style="direction: ltr">-->
  <!--              <b-button-->
  <!--                class="align-self-end"-->
  <!--                variant="outline-danger"-->
  <!--                size="sm"-->
  <!--                @click="removeSemester"-->
  <!--              >-->
  <!--                מחק סמסטר-->
  <!--              </b-button>-->
  <!--              <b-button-->
  <!--                v-if="semester.name.toString().includes('קיץ')"-->
  <!--                class="align-self-end"-->
  <!--                size="sm"-->
  <!--                variant="outline-info"-->
  <!--                @click="changeToRegular"-->
  <!--              >-->
  <!--                הפוך לסמסטר רגיל-->
  <!--              </b-button>-->
  <!--              <b-button-->
  <!--                v-else-->
  <!--                class="align-self-end"-->
  <!--                size="sm"-->
  <!--                variant="outline-info"-->
  <!--                @click="changeToSummer"-->
  <!--              >-->
  <!--                הפוך לסמסטר קיץ-->
  <!--              </b-button>-->
  <!--            </b-button-group>-->
  <!--          </div>-->
  <!--        </div>-->
  <!--      </b-tab>-->

  <!--      &lt;!&ndash; New Tab Button (Using tabs slot) &ndash;&gt;-->
  <!--      <template v-slot:tabs-end>-->
  <!--        <b-nav-item href="#"-->
  <!--                    @click.prevent="newTab">-->
  <!--          <b>+</b>-->
  <!--        </b-nav-item>-->
  <!--      </template>-->

  <!--      &lt;!&ndash; Render this if no tabs &ndash;&gt;-->
  <!--      <div-->
  <!--        slot="empty"-->
  <!--        class="container justify-content-md-center alert alert-secondary text-center text-muted"-->
  <!--      >-->
  <!--        <h2>עוד לא נוספו סמסטרים</h2>-->

  <!--        <br />-->

  <!--        <b-button variant="outline-secondary"-->
  <!--                  @click.prevent="newTab">-->
  <!--          הוסף סמסטר-->
  <!--        </b-button>-->
  <!--      </div>-->
  <!--    </b-tabs>-->
  <!--  </b-card>-->
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

export default defineComponent({
  name: "SemestersTabView",
  components: { AppSemesterTable },
  setup() {
    const store = useStore();
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
    const semesters = computed(() => {
      return store.getters[USER_STORE.GETTERS.SEMESTERS];
    });
    const newTab = () => {
      store.commit(USER_STORE.MUTATIONS.addSemester, 1);
    };
    const changeToSummer = () => {
      store.commit(USER_STORE.MUTATIONS.changeActiveSemesterType);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    const changeToRegular = () => {
      store.commit(USER_STORE.MUTATIONS.changeActiveSemesterType);
      store.dispatch(USER_STORE.ACTIONS.updateSemesterAsync);
    };
    return {
      semesters,
      removeSemester,
      newTab,
      changeToSummer,
      changeToRegular,
    };
  },
  mounted() {
    const store = useStore();
    let authentication_status = localStorage.getItem("authenticated");
    const user = firebase.auth().currentUser;
    if (user == null) {
      if (authentication_status === "false") {
        let user_data = localStorage.getItem("saved_session_data");
        if (user_data == null) {
          return;
        }
        if (typeof user_data === "object") {
          store.commit(USER_STORE.MUTATIONS.fetchUserInfo, user_data);
        } else {
          let session_data = localStorage.getItem("saved_session_data");
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
