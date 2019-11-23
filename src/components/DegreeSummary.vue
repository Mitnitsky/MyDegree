<template>
  <div class=" justify-content-md-center   text-center text-muted"
       style="margin-bottom: 60px;margin-right: 5px;">
    <b-button @click="collapsed = !collapsed"
              style="margin: 5px;"
              v-b-toggle.collapse-summary
              v-if="collapsed"
              variant="outline-dark">הראה סיכום תואר &Darr;
    </b-button>
    <b-button @click="collapsed = !collapsed"
              style="margin: 5px"
              v-b-toggle.collapse-summary
              v-if="!collapsed"
              variant="outline-dark">הסתר סיכום תואר &Uarr;
    </b-button>
    <b-collapse id="collapse-summary">
      <b-card-group >
        <div class="container-fluid">
          <div class="row">
            <div class="col-xl-2"></div>
            <div class="col-xl-4">
              <b-card flow
                      header="סיכום תואר"
                      header-bg-variant="dark"
                      header-text-variant="white"
                     >
                <div class="input-group mb-2"
                     style="margin-top: 46px">
                  <div class="input-group categoryName">
                    <span class="input-group-text categoryNameSpan">נקודות תואר</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         type="number"
                         v-model.number="degree_points">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group categoryName">
                    <span class="input-group-text categoryNameSpan">ממוצע תואר</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.01"
                         type="number"
                         v-model.number="degree_average">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group categoryName">
                    <span class="input-group-text categoryNameSpan">נקודות בוצעו</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="degree_points_done">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group categoryName">
                    <span class="input-group-text categoryNameSpan">נקודות נותרו</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="degree_points_left">
                  <div class="input-group degree-summary degree-summary">

                  </div>
                </div>
                <div class="input-group mb-2">
                  <div class="input-group categoryName">
                    <span class="input-group-text categoryNameSpan">נותרו לשבץ</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="degree_points_to_choose">
                  <div class="input-group degree-summary degree-summary">
                  </div>
                </div>
              </b-card>
            </div>
            <div class="col-xl-4 ">
              <b-card flow
                      class="h-100"
                      header="ניתוח סוגי קורסים"
                      header-bg-variant="dark"
                      header-text-variant="white">
                <div class="input-group mb-2">
                  <div class="input-group courseName">

                  </div>
                  <input class="input-group form-control"
                         readonly
                         style="background-color: aliceblue;align-content: center;text-align: center;"
                         type="text"
                         value="נותרו:"
                  >
                  <input class="input-group form-control"
                         readonly
                         style="background-color: aliceblue;text-align: center;"
                         type="text"
                         value="מתוך:">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">חובה </span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         type="number"
                         step="0.5"
                         v-model.number="must_points_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="must_points">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">רשימה א'</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         type="number"
                         step="0.5"
                         v-model.number="a_list_points_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="a_list_points">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">רשימה ב'</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         type="number"
                         step="0.5"
                         v-model.number="b_list_points_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="b_list_points">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">הומניסטיים</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         type="number"
                         v-model.number="humanistic_points_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="humanistic_points">
                </div>
                <div class="input-group mb-2">
                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">בחירה חופשית</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="free_points_left"
                  >
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="free_points">
                </div>
                <div class="input-group mb-2">

                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">פרוייקטים</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="projects_points_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="projects_points">
                </div>
                <div class="input-group mb-2">

                  <div class="input-group courseName">
                    <span class="input-group-text courseNameSpan">ספורט</span>
                  </div>
                  <input class="form-control degree-summary degree-summary-number disabled-input"
                         disabled="disabled"
                         readonly
                         step="0.5"
                         type="number"
                         v-model.number="sport_left">
                  <input class="form-control degree-summary degree-summary-number degree-input-field"
                         max="9999999"
                         min="0"
                         step="0.5"
                         style="text-align: center"
                         type="number"
                         v-model.number="sport">
                </div>
                <div class="input-group mb-2">
                  <b-form-checkbox
                      id="checkbox-1"
                      name="checkbox-1"
                      v-model.number="english_exemption"
                  >
                    פטור מאנגלית
                  </b-form-checkbox>
                </div>
              </b-card>
            </div>
            <div class="col-xl-2"></div>
          </div>
        </div>
      </b-card-group>
    </b-collapse>
  </div>
</template>
<script>
    import {createHelpers} from 'vuex-map-fields';

    // The getter and mutation types we're providing
    // here, must be the same as the function names we've
    // used in the store.
    const {mapFields} = createHelpers({
        getterType: 'getUserField',
        mutationType: 'updateUserField',
    });
    export default {
        name: 'degree-summary',
        data() {
            return {
                collapsed: true,
                buttonText: "Show summary",
                inputIsWrong: 'inputIsWrong'
            }
        },
        computed: {
            ...mapFields([
                'token',
                'active_semester',
                'degree_average',
                'degree_points',
                'degree_points_done',
                'degree_points_left',
                'degree_points_to_choose',
                'must_points',
                'must_points_left',
                'a_list_points',
                'a_list_points_left',
                'b_list_points',
                'b_list_points_left',
                'humanistic_points',
                'humanistic_points_left',
                'free_points',
                'free_points_left',
                'projects_points',
                'projects_points_left',
                'sport',
                'sport_left',
                'exemption_points',
                'english_exemption',
                'semesters'
            ]),
        },
        methods: {},
    }
</script>

<style scoped>
  input[type=number]::-webkit-inner-spin-button,
  input[type=number]::-webkit-outer-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .courseName {
    width: 30%;
  }

  .courseNameSpan {
    width: 100%;
    background-color: aliceblue;
  }

  .left {

  }
  .inputIsWrong{
    border-color: red !important;
  }
  .toDo {

  }
  .disabled-input{
    background-color: whitesmoke !important;
  }
  .degree-input-field:hover{
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
    width: 100%;
    background-color: aliceblue;
  }

  .rowSpace {
    margin-bottom: 2px
  }
  .card{
    height: 100%;
  }
</style>