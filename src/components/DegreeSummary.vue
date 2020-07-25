<template>
  <div
    class=" justify-content-md-center   text-center text-muted"
    style="margin-bottom: 60px;margin-right: 5px;"
  >
    <b-button
      v-if="collapsed"
      v-b-toggle.collapse-summary
      style="margin: 5px;"
      variant="outline-dark"
      @click="collapsed = !collapsed"
    >
      הראה סיכום תואר &Darr;
    </b-button>
    <b-button
      v-if="!collapsed"
      v-b-toggle.collapse-summary
      style="margin: 5px"
      variant="outline-dark"
      @click="collapsed = !collapsed"
    >
      הסתר סיכום תואר &Uarr;
    </b-button>
    <b-collapse id="collapse-summary">
      <b-card-group deck>
        <div class="container justify-content-center">
          <div class="row justify-content-center">
            <div
              class="col "
              style="max-width: 590px; min-width: 480px"
            >
              <b-card
                class="shadow bg-white rounded h-100"
                flow
                header="סיכום תואר"
                header-bg-variant="dark"
                header-class="summary-card-header"
                header-text-variant="white"
                style=""
              >
                <div style="height: 100%; margin-top: 46px">
                  <div class="input-group mb-2">
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >נקודות תואר</span>
                    <input
                      v-model.number="degree_points"
                      v-b-tooltip.hover.v-dark
                      class="form-control degree-summary degree-summary-number degree-input-field"
                      max="9999999"
                      min="0"
                      step="0.5"
                      title="יש למלא שדה זה"
                      type="number"
                    >
                  </div>

                  <div class="input-group mb-2">
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >ממוצע תואר</span>
                    <input
                      v-model.number="degree_average"
                      class="form-control degree-summary degree-summary-number disabled-input"
                      disabled="disabled"
                      max="100"
                      min="0"
                      readonly
                      step="0.01"
                      type="number"
                    >
                  </div>
                  <div class="input-group mb-2">
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >נקודות בוצעו</span>
                    <input
                      v-model.number="degree_points_done"
                      class="form-control degree-summary degree-summary-number disabled-input"
                      disabled="disabled"
                      readonly
                      step="0.5"
                      type="number"
                    >
                  </div>
                  <div class="input-group mb-2">
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >נקודות נותרו</span>
                    <input
                      v-model.number="degree_points_left"
                      class="form-control degree-summary degree-summary-number disabled-input"
                      disabled="disabled"
                      readonly
                      step="0.5"
                      type="number"
                    >
                  </div>
                  <div class="input-group mb-2">
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >נותרו לשבץ</span>
                    <input
                      v-model.number="degree_points_to_choose"
                      class="form-control degree-summary degree-summary-number disabled-input"
                      disabled="disabled"
                      readonly
                      step="0.5"
                      type="number"
                    >
                  </div>
                </div>
              </b-card>
            </div>
            <div
              class="col "
              style="max-width: 590px; min-width: 480px"
            >
              <b-card
                class="h-100 shadow bg-white rounded"
                flow
                header="ניתוח סוגי קורסים"
                header-bg-variant="dark"
                header-class="summary-card-header"
                header-text-variant="white"
              >
                <div class="input-group mb-2">
                  <input
                    class="input-group-prepend input-group-addon form-control"
                    disabled
                    readonly
                    style="background-color: aliceblue;align-content: center;text-align: center; margin-right: 33.6%"
                    type="text"
                    value="נותרו"
                  >
                  <input
                    class="input-group  form-control column-headers"
                    disabled
                    readonly
                    style="background-color: aliceblue;text-align: center;"
                    title="יש למלא שדות אלו בהתאם לתואר"
                    type="text"
                    value="מתוך"
                  >
                </div>

                <template v-for="(type, index) in course_types">
                  <div
                    v-if="type.name !== 'פטור'"
                    :key="index"
                    class="input-group mb-2"
                  >
                    <span
                      class="input-group-text categoryNameSpan"
                      style="width: 33%"
                    >{{ type.name }}</span>
                    <input
                      v-model.number="type.points_left"
                      class="input-group-append form-control degree-summary disabled-input"
                      dir="ltr"
                      disabled="disabled"
                      readonly
                      step="0.5"
                      type="number"
                    >
                    <input
                      v-model.number="type.points_required"
                      v-b-tooltip.hover.left.v-dark
                      class="input-group-append form-control degree-summary degree-summary-number degree-input-field"
                      dir="ltr"
                      max="9999999"
                      min="0"
                      step="0.5"
                      title="יש למלא שדה זה"
                      type="number"
                      @input="updateInfo"
                    >
                  </div>
                </template>

                <div class="input-group mb-2">
                  <b-form-checkbox
                    id="checkbox-1"
                    v-model.number="english_exemption"
                    name="checkbox-1"
                  >
                    פטור מאנגלית
                  </b-form-checkbox>
                </div>
              </b-card>
            </div>
          </div>
        </div>
      </b-card-group>
    </b-collapse>
  </div>
</template>
<script>
import { createHelpers } from "vuex-map-fields";

const { mapFields } = createHelpers({
  getterType: "getUserField",
  mutationType: "updateUserField"
});

export default {
  name: "DegreeSummary",
  data() {
    return {
      collapsed: true,
      buttonText: "Show summary",
      inputIsWrong: "inputIsWrong"
    };
  },
  computed: {
    ...mapFields([
      "token",
      "active_semester",
      "degree_average",
      "degree_points",
      "degree_points_done",
      "degree_points_left",
      "degree_points_to_choose",
      "english_exemption",
      "semesters",
      "course_types"
    ])
  },
  methods: {
    updateInfo() {
      this.$store.commit("reCalcCurrentSemester");
    }
  }
};
</script>

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
  border-bottom-right-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
  border-bottom-left-radius: 0;
  border-top-left-radius: 0;
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
  border-bottom-right-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
  border-bottom-left-radius: 0;
  border-top-left-radius: 0;
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
  border-bottom-right-radius: 0;
  border-top-right-radius: 0;
  border-bottom-left-radius: 0.25rem;
  border-top-left-radius: 0.25rem;
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
  border-bottom-right-radius: 0;
  border-top-right-radius: 0;
  border-bottom-left-radius: 0.25rem;
  border-top-left-radius: 0.25rem;
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
</style>
