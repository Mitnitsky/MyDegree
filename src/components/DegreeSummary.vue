<template>
  <el-row justify="center" style="min-width: 500px">
    <el-collapse style="margin: 5px;"
                 @change="collapsed = !collapsed">
      <el-collapse-item
        :title="collapsed ? 'הראה סיכום תואר' : 'הסתר סיכום תואר'"
      >
        <el-row>
          <el-col :span="1" />
          <el-col :span="10">
            <el-card style="height: 100%;max-width: 590px !important; min-width: 340px !important;"
                     :body-style="{ padding: '20px !important' }">
              <template #header>
                <span style="font-weight: 700 !important">סיכום תואר</span>
              </template>
              <el-row class="input-group mb-2"
                      style="margin-top: 46px;">
                <el-col :span="8">
                  <el-tooltip
                    class="box-item"
                    effect="dark"
                    content="כל הנקודות שיש לבצע בתואר"
                    placement="top-start"
                    type="text"
                  >
                    <el-input
                      model-value="נקודות תואר"
                      class="first-disabled-element"
                    />
                  </el-tooltip>
                </el-col>
                <el-col :span="16">
                  <el-tooltip
                    class="box-item"
                    effect="dark"
                    content="יש למלא שדה זה בהתאם למסלול הלימודים"
                    placement="top-start"
                  >
                    <el-input
                      v-model.number="degree_points"
                      class="last-input-element form-control degree-summary degree-summary-number degree-input-field"
                      max="9999999"
                      min="0"
                      step="0.5"
                      type="number"
                    />
                  </el-tooltip>
                </el-col>
              </el-row>
              <el-row class="input-group mb-2">
                <el-col :span="8">
                  <el-input
                    model-value="ממוצע תואר"
                    class="first-disabled-element"
                  />
                </el-col>
                <el-col :span="16">
                  <el-input
                    v-model.number="degree_average"
                    class="last-disabled-element form-control degree-summary degree-summary-number degree-input-field"
                    disabled="disabled"
                    max="100"
                    min="0"
                    readonly
                    step="0.01"
                    type="number"
                  />
                </el-col>
              </el-row>
              <el-row class="input-group mb-2">
                <el-col :span="8">
                  <el-tooltip
                    class="box-item"
                    effect="dark"
                    content="נקודות עם ציון\פטור\בינטארי בתואר"
                    placement="top-start"
                    type="text"
                  >
                    <el-input
                      model-value="נקודות בוצעו"
                      class="first-disabled-element"
                    />
                  </el-tooltip>
                </el-col>
                <el-col :span="16">
                  <el-input
                    v-model.number="degree_points_done"
                    class="last-disabled-element form-control degree-summary degree-summary-number degree-input-field"
                    disabled="disabled"
                    max="100"
                    min="0"
                    readonly
                    step="0.01"
                    type="number"
                  />
                </el-col>
              </el-row>
              <el-row class="input-group mb-2">
                <el-col :span="8">
                  <el-tooltip
                    class="box-item"
                    effect="dark"
                    content="כל הנקודות הדרושות בתואר פחות הנקודות שבוצעו"
                    placement="top-start"
                    type="text"
                  >
                    <el-input
                      model-value="נקודות נותרו"
                      class="first-disabled-element"
                    />
                  </el-tooltip>
                </el-col>
                <el-col :span="16">
                  <el-input
                    v-model.number="degree_points_left"
                    class="last-disabled-element form-control degree-summary degree-summary-number degree-input-field"
                    disabled="disabled"
                    max="100"
                    min="0"
                    readonly
                    step="0.01"
                    type="number"
                  />
                </el-col>
              </el-row>
              <el-row class="input-group mb-2">
                <el-col :span="8">
                  <el-tooltip
                    class="box-item"
                    effect="dark"
                    content="כל הנקודות הדרושות בתואר פחות הנקודות שקיימות בתכנון התואר"
                    placement="top-start"
                    type="text"
                  >
                    <el-input
                      model-value="נותרו לשבץ"
                      class="first-disabled-element"
                    />
                  </el-tooltip>
                </el-col>
                <el-col :span="16">
                  <el-input
                    v-model.number="degree_points_to_choose"
                    class="last-disabled-element form-control degree-summary degree-summary-number degree-input-field"
                    disabled="disabled"
                    max="100"
                    min="0"
                    readonly
                    step="0.01"
                    type="number"
                  />
                </el-col>
              </el-row>
            </el-card>
          </el-col>
          <el-col :span="2" />
          <el-col :span="10">
            <el-card style="height: 100%;max-width: 590px !important; min-width: 340px !important;"
                     :body-style="{ padding: '20px !important' }">
              <template #header>
                <span style="font-weight: 700 !important">ניתוח סוגי קורסים</span>
              </template>
              <el-row class="input-group">
                <el-col :span="8" />
                <el-col :span="8">
                  <el-input
                    class="disabled-inputs first-disabled-element"
                    type="text"
                    model-value="נותרו"
                  />
                </el-col>
                <el-col :span="8">
                  <el-input
                    class="disabled-inputs last-disabled-element"
                    type="text"
                    model-value="מתוך"
                  />
                </el-col>
              </el-row>

              <template v-for="(type, index) in sortCourseTypes(course_types)">
                <el-row
                  v-if="
                  type.name !== 'פטור' ||
                  (type.name === 'פטור' && type.total_points > 0)
                "
                  :key="index"
                  class="input-group"
                >
                  <el-col :span="8">
                    <el-tooltip>
                      <template #content
                      >סה"כ נקודות משובצות: {{ type.total_points }}<br />
                       נקודות עם ציון: {{ type.points_done }}
                        <br v-if="type.average > 0" />
                       {{ type.average > 0 ? "ממוצע " + type.average : "" }}
                      </template>
                      <el-input
                        class="disabled-inputs first-disabled-element"
                        type="text"
                        :model-value="type.name"
                      />
                    </el-tooltip>
                  </el-col>
                  <el-col v-if="type.name !== 'פטור'"
                          :span="8">
                    <el-input
                      class="disabled-inputs middle-disabled-element"
                      type="text"
                      disabled
                      :model-value="type.points_left"
                    />
                  </el-col>
                  <el-col :span="type.name !== 'פטור' ? 8 : 16">
                    <template v-if="type.name !== 'פטור'">
                      <el-tooltip content="יש למלא שדה זה בהתאם למסלול הלימודים">
                        <el-input
                          class="last-input-element"
                          type="text"
                          :model-value="type.points_required"
                        />
                      </el-tooltip>
                    </template>
                    <template v-else>
                      <el-tooltip content="נקודות פטור קיימות">
                        <el-input
                          v-model.number="type.points_done"
                          class="last-input-element"
                          disabled
                          @input="updateInfo"
                        />
                      </el-tooltip>
                    </template>
                  </el-col>
                </el-row>
              </template>

              <el-row class="input-group">
                <el-tooltip content="נקודות יורדות מהחובה אוטומטית">
                  <el-checkbox
                    id="checkbox-1"
                    v-model.number="english_exemption"
                    name="checkbox-1"
                  >
                    פטור מאנגלית
                  </el-checkbox>
                </el-tooltip>
              </el-row>
            </el-card>
          </el-col>
          <el-col :span="1" />
        </el-row>
      </el-collapse-item>
    </el-collapse>
  </el-row>
</template>
<script lang="ts">
import { useStore } from "@/use/useStore";
import { USER_STORE } from "@/store/constants";
import { ref, computed, defineComponent } from "vue";
import { CourseType } from "@/store/classes/course_types";

export default defineComponent({
  name: "DegreeSummary",

  setup() {
    const store = useStore();
    const collapsed = ref(true);
    const buttonText = ref("Show summary");
    const inputIsWrong = ref("inputIsWrong");
    const degree_average = computed<number>({
      get(): number {
        return store.getters[USER_STORE.GETTERS.DEGREE_AVERAGE];
      },
      set(average: number): void {
        store.commit(USER_STORE.MUTATIONS.setDegreeAverage, average);
      },
    });
    const degree_points = computed<number>({
      get(): number {
        return store.getters[USER_STORE.GETTERS.DEGREE_POINTS];
      },
      set(average: number): void {
        store.commit(USER_STORE.MUTATIONS.setDegreePoints, average);
      },
    });
    const degree_points_done = computed<number>({
      get(): number {
        return store.getters[USER_STORE.GETTERS.DEGREE_POINTS_DONE];
      },
      set(average: number): void {
        store.commit(USER_STORE.MUTATIONS.setDegreePointsDone, average);
      },
    });
    const degree_points_left = computed<number>({
      get(): number {
        return store.getters[USER_STORE.GETTERS.DEGREE_POINTS_LEFT];
      },
      set(average: number): void {
        store.commit(USER_STORE.MUTATIONS.setDegreePointsLeft, average);
      },
    });
    const degree_points_to_choose = computed<number>({
      get(): number {
        return store.getters[USER_STORE.GETTERS.DEGREE_POINTS_TO_CHOOSE];
      },
      set(average: number): void {
        store.commit(USER_STORE.MUTATIONS.setDegreePointsToChoose, average);
      },
    });
    const english_exemption = computed<boolean>({
      get(): boolean {
        return store.getters[USER_STORE.GETTERS.ENGLISH_EXEMPTION];
      },
      set(status: boolean): void {
        store.commit(USER_STORE.MUTATIONS.setExemptionStatus, status);
      },
    });
    const course_types = computed<CourseType[]>(() => {
      return store.getters[USER_STORE.GETTERS.COURSE_TYPES];
    });
    const updateInfo = () => {
      store.commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
    };
    const sortCourseTypes = (course_types) => {
      let sorted_course_types: CourseType[] = [];
      let exemption_course_type: CourseType | null = null;
      for (let type of course_types) {
        if (type.name.includes("פטור")) {
          exemption_course_type = type;
        } else {
          sorted_course_types.push(type);
        }
      }
      if (exemption_course_type) {
        sorted_course_types.push(exemption_course_type);
      }
      return sorted_course_types;
    };
    return {
      degree_average,
      degree_points_done,
      degree_points_left,
      degree_points_to_choose,
      english_exemption,
      degree_points,
      course_types,
      collapsed,
      buttonText,
      inputIsWrong,
      updateInfo,
      sortCourseTypes,
    };
  },
});
</script>

<style>
span.el-checkbox__label {
  padding-left: 0;
  padding-right: 8px;
}
.el-collapse-item__header{
  font-weight: 700 !important;
}
div.middle-disabled-element > input.el-input__inner {
  text-align: center;
  cursor: default !important;
  color: black;
  border-radius: 0;
}

div.first-disabled-element > input.el-input__inner {
  text-align: center;
  background-color: aliceblue;
  color: black;
  cursor: default;
  border-top-left-radius: 0;
  border-bottom-left-radius: 0;
}

div.last-input-element > input.el-input__inner {
  text-align: center;
  color: black;
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

div.last-disabled-element > input.el-input__inner {
  text-align: center;
  color: black;
  cursor: default !important;
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
}

div.disabled-inputs > input.el-input__inner {
  text-align: center;
  cursor: default;
  background-color: aliceblue;
  align-content: center;
  margin-bottom: 8px;
}
.el-collapse-item__header{
  display: block !important;
  align-content: center !important;
  text-align: center !important;
}
</style>
<style scoped>
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.courseName {
  width: 30%;
}

.courseNameSpan {
  width: 100%;
  background-color: aliceblue;
  cursor: default;
}

.disabled-input {
  background-color: whitesmoke !important;
}

.degree-input-field:hover {
  border-color: royalblue !important;
}

.degree-summary {
  text-align: center;
}

.degree-summary-number {
  direction: ltr;
}

.categoryName {
  width: 130px;
}

.categoryNameSpan {
  width: 33%;
  background-color: aliceblue;
  cursor: default;
}

.courseNameDiV {
  border-radius: 0.25rem 0 0 0.25rem !important;
}

.summary-card-header {
  font-weight: bold !important;
}

/*Thanks to Vucko at https://stackoverflow.com/questions/42677620/bootstrap-4-input-group-rtl-issue*/
[dir="rtl"] .input-group-addon:not(:last-child) {
  border-right: 1px solid rgba(0, 0, 0, 0.15);
  border-left: 0;
}

[dir="rtl"] .input-group-text:not(:last-child) {
  border-right: 1px solid rgba(0, 0, 0, 0.15);
  border-left: 0;
}

[dir="rtl"] .input-group .form-control:not(:last-child),
[dir="rtl"] .input-group-text:not(:last-child),
[dir="rtl"]
  .input-group-btn:not(:first-child)
  > .btn-group:not(:last-child)
  > .btn,
[dir="rtl"]
  .input-group-btn:not(:first-child)
  > .btn:not(:last-child):not(.dropdown-toggle),
[dir="rtl"] .input-group-btn:not(:last-child) > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .btn-group > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .dropdown-toggle {
  border-radius: 0 0.25rem 0.25rem 0;
}

[dir="rtl"] .input-group .form-control:not(:last-child),
[dir="rtl"] .input-group-addon:not(:last-child),
[dir="rtl"]
  .input-group-btn:not(:first-child)
  > .btn-group:not(:last-child)
  > .btn,
[dir="rtl"]
  .input-group-btn:not(:first-child)
  > .btn:not(:last-child):not(.dropdown-toggle),
[dir="rtl"] .input-group-btn:not(:last-child) > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .btn-group > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .dropdown-toggle {
  border-radius: 0 0.25rem 0.25rem 0;
}

[dir="rtl"] .input-group .form-control:not(:first-child),
[dir="rtl"] .input-group-addon:not(:first-child),
[dir="rtl"] .input-group-btn:not(:first-child) > .btn,
[dir="rtl"] .input-group-btn:not(:first-child) > .btn-group > .btn,
[dir="rtl"] .input-group-btn:not(:first-child) > .dropdown-toggle,
[dir="rtl"]
  .input-group-btn:not(:last-child)
  > .btn-group:not(:first-child)
  > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .btn:not(:first-child) {
  border-radius: 0.25rem 0 0 0.25rem;
}

[dir="rtl"] .input-group .form-control:not(:first-child),
[dir="rtl"] .input-group-text:not(:first-child),
[dir="rtl"] .input-group-btn:not(:first-child) > .btn,
[dir="rtl"] .input-group-btn:not(:first-child) > .btn-group > .btn,
[dir="rtl"] .input-group-btn:not(:first-child) > .dropdown-toggle,
[dir="rtl"]
  .input-group-btn:not(:last-child)
  > .btn-group:not(:first-child)
  > .btn,
[dir="rtl"] .input-group-btn:not(:last-child) > .btn:not(:first-child) {
  border-radius: 0.25rem 0 0 0.25rem;
}

[dir="rtl"] .form-control + .input-group-addon:not(:first-child) {
  border-left: 1px solid rgba(0, 0, 0, 0.15);
  border-right: 0;
}

[dir="rtl"] .form-control + .input-group-text:not(:first-child) {
  border-left: 1px solid rgba(0, 0, 0, 0.15);
  border-right: 0;
}

[dir="rtl"] .input-group .form-control:not(:first-child):not(:last-child),
[dir="rtl"] .input-group .input-group-addon:not(:first-child):not(:last-child) {
  border-radius: 0;
}

[dir="rtl"] .input-group .form-control:not(:first-child):not(:last-child),
[dir="rtl"] .input-group .input-group-text:not(:first-child):not(:last-child) {
  border-radius: 0;
}

.input-group > .input-group-prepend {
  flex: 0 0 33%;
}

.input-group .input-group-text {
  width: 100%;
}

[dir="rtl"] .input-group .form-control:not(:first-child):not(:last-child),
[dir="rtl"] .input-group .input-group-addon:not(:first-child):not(:last-child) {
  border-radius: 0;
}

[dir="rtl"] .input-group .form-control:not(:first-child):not(:last-child),
[dir="rtl"] .input-group .input-group-text:not(:first-child):not(:last-child) {
  border-radius: 0;
}
.mb-2{
  margin-bottom: 8px;
}
</style>
