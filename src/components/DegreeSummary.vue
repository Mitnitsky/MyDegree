<template>
  <div class="container justify-content-md-center alert alert-secondary text-center text-muted"
       style="margin-bottom: 60px">
    <b-button @click="collapsed = !collapsed"
              style="margin: 5px;"
              v-b-toggle.collapse-summary
              v-if="collapsed">הראה סיכום תואר &Darr;
    </b-button>
    <b-collapse id="collapse-summary"
                style="margin: 5px; ">
      <div class="row">
        <div class="col-sm-5">
          <h3 style="text-decoration: underline; ">סיכום תואר</h3>
          <div class="input-group mb-2"
               style="margin-top: 62px">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות תואר</span>
            </div>
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   type="number"
                   v-model="degreePoints">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">ממוצע תואר</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="degree_average">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות בוצעו</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="degree_points_done">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות נותרו</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="degree_points_left">
            <div class="input-group degree-summary degree-summary">

            </div>
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נותרו לשבץ</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="degree_points_to_choose">
            <div class="input-group degree-summary degree-summary">
            </div>
          </div>

        </div>
        <div class="col-sm-1">
        </div>
        <div class="col-sm-6">
          <h3 style="text-decoration: underline; ">ניתוח סוגי קורסים</h3>
          <div class="input-group mb-2">
            <div class="input-group courseName">

            </div>
            <input class="input-group form-control"
                   readonly
                   style="background-color: aliceblue;align-content: center;"
                   type="text"
                   value="נותרו"
            >
            <input class="input-group form-control"
                   readonly
                   style="background-color: aliceblue"
                   type="text"
                   value="מתוך">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">חובה: </span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="must_points_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="must_points">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">רשימה א'</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="a_list_points_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="a_list_points">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">רשימה ב'</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="b_list_points_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="b_list_points">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">הומניסטיים</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="humanistic_points_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="humanistic_points">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">בחירה חופשית:</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="free_points_left"
            >
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="free_points">
          </div>
          <div class="input-group mb-2">

            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">פרוייקטים:</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="projects_points_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="projects_points">
          </div>
          <div class="input-group mb-2">

            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">ספורט:</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text"
                   v-model="sport_left">
            <input class="form-control degree-summary"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number"
                   v-model="sport">
          </div>
          <div class="input-group mb-2">
            <b-form-checkbox
                id="checkbox-1"
                name="checkbox-1"
                v-model="english_exemption"
            >
              פטור מאנגלית
            </b-form-checkbox>
          </div>
        </div>
      </div>
    </b-collapse>
    <b-button @click="collapsed = !collapsed"
              style="margin: 5px"
              v-b-toggle.collapse-summary
              v-if="!collapsed">הסתר סיכום תואר &Uarr;
    </b-button>

  </div>
</template>
<script>
    export default {
        name: 'degree-summary',
        data() {
            return {
                collapsed: true,
                buttonText: "Show summary"
            }
        },
        computed: {
            degreePoints: {
                get() {
                    return this.$store.state.user.degree_points
                },
                set(points) {
                    this.$store.commit('updateDegreePoints', points);
                    this.$store.commit('reCalcCurrentSemester');
                }
            },
            degree_average: {
                get() {
                    if (this.$store.state.user.degree_average === 0) {
                        return ''
                    } else {
                        return this.$store.state.user.degree_average;
                    }
                }
            },
            degree_points_done: {
                get() {
                    return this.$store.state.user.degree_points_done;
                }
            },
            degree_points_left: {
                get() {
                    return this.$store.state.user.degree_points_left;
                }
            },
            degree_points_to_choose: {
                get() {
                    return this.$store.state.user.degree_points_to_choose;
                }
            },
            must_points: {
                get() {
                    return this.$store.state.user.must_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'must_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            must_points_left: {
                get() {
                    return this.$store.state.user.must_points_left;
                }
            },
            a_list_points: {
                get() {
                    return this.$store.state.user.a_list_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'a_list_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            a_list_points_left: {
                get() {
                    return this.$store.state.user.a_list_points_left;
                }
            },
            b_list_points: {
                get() {
                    return this.$store.state.user.b_list_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'b_list_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            b_list_points_left: {
                get() {
                    return this.$store.state.user.b_list_points_left;
                }
            },
            humanistic_points: {
                get() {
                    return this.$store.state.user.humanistic_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'humanistic_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            humanistic_points_left: {
                get() {
                    return this.$store.state.user.humanistic_points_left;
                }
            },
            free_points: {
                get() {
                    return this.$store.state.user.free_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'free_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            free_points_left: {
                get() {
                    return this.$store.state.user.free_points_left;
                }
            },
            projects_points: {
                get() {
                    return this.$store.state.user.projects_points;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'projects_points', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            projects_points_left: {
                get() {
                    return this.$store.state.user.projects_points_left;
                }
            },
            sport: {
                get() {
                    return this.$store.state.user.sport;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'sport', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            },
            sport_left: {
                get() {
                    return this.$store.state.user.sport_left;
                }
            },
            english_exemption: {
                get() {
                    return this.$store.state.user.english_exemption;
                },
                set(value) {
                    this.$store.commit('updateInfo', {field: 'english_exemption', value});
                    this.$store.commit('reCalcCurrentSemester')
                }
            }
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

  .toDo {

  }

  .degree-summary {
    text-align: center
  }

  .categoryName {
    width: 40%;
  }

  .categoryNameSpan {
    width: 100%;
    background-color: aliceblue;
  }

  .rowSpace {
    margin-bottom: 2px
  }

</style>