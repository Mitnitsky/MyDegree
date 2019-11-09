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
        <div class="col-sm-4">
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
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">ממוצע תואר</span>
            </div>
            <input class="form-control degree-summary"
                   disabled="disabled"
                   readonly
                   type="text">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות הושלמו</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות נותרו</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <div class="input-group degree-summary degree-summary">

            </div>
          </div>
          <div class="input-group mb-2">
            <div class="input-group categoryName">
              <span class="input-group-text categoryNameSpan">נקודות לבחור</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <div class="input-group degree-summary degree-summary">

            </div>
          </div>

        </div>
        <div class="col-sm-2">
        </div>
        <div class="col-sm-6">
          <h3 style="text-decoration: underline; ">סיכום נקודות</h3>
          <div class="input-group mb-2">
            <div class="input-group courseName">

            </div>
            <input class="input-group form-control"
                   readonly
                   style="background-color: aliceblue;align-content: center;"
                   type="text"
                   value="נותרו">
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
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">רשימה א'</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">רשימה ב'</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">הומניסטיים</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">בחירה חופשית:</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">

            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">פרוייקטים:</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">

            <div class="input-group courseName">
              <span class="input-group-text courseNameSpan">ספורט:</span>
            </div>
            <input class="form-control"
                   disabled="disabled"
                   readonly
                   type="text">
            <input class="form-control"
                   max="9999999"
                   min="0"
                   step="1"
                   style="text-align: center"
                   type="number">
          </div>
          <div class="input-group mb-2">
            <b-form-checkbox
                id="checkbox-1"
                name="checkbox-1"
                v-model="englishExemption"
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
                englishExemption: false,
                collapsed: true,
                buttonText: "Show summary"
            }
        }
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