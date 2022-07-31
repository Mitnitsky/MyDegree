<template>
  <div class="container" style="max-width: 300px">
    <div class="col align-self-start" style="padding: 0 0">
      <b-card no-body>
        <b-card-header
          class="text-center"
          style="
            padding-bottom: 3px;
            padding-top: 3px;
            background-color: #e9ecef !important;
            color: #495057 !important;
          "
        >
          <p style="color: #495057; margin-bottom: 0; font-weight: bold">
            סיכום סמסטר
          </p>
        </b-card-header>
        <b-card-body>
          <b-row>
            <b-col align-self="center" sm="3" style="margin: 4px">
              <label
                :for="
                  'semester' + $store.state.user.active_semester + 'average'
                "
                >ממוצע:
              </label>
            </b-col>
            <b-col sm="8">
              <b-input
                :id="'semester' + $store.state.user.active_semester + 'average'"
                v-model.number="
                  $store.state.user.semesters[$store.state.user.active_semester]
                    .average
                "
                max="100"
                min="0"
                readonly
                style="
                  text-align: center;
                  direction: ltr;
                  cursor: default;
                  margin-left: 12px;
                  margin-right: 12px;
                "
                type="number"
              />
            </b-col>
          </b-row>
          <b-row class="mt-2">
            <b-col align-self="center" sm="3" style="margin: 4px">
              <label
                :for="'semester' + $store.state.user.active_semester + 'points'"
                >נקודות:
              </label>
            </b-col>
            <b-col sm="8">
              <b-input
                :id="'semester' + $store.state.user.active_semester + 'points'"
                v-model.number="
                  $store.state.user.semesters[$store.state.user.active_semester]
                    .points
                "
                max="100"
                min="0"
                readonly
                style="
                  text-align: center;
                  direction: ltr;
                  cursor: default;
                  margin-left: 12px;
                  margin-right: 12px;
                "
                type="number"
              />
            </b-col>
          </b-row>
        </b-card-body>
      </b-card>
    </div>
  </div>
</template>

<script>
export default {
  name: "SemesterSummary",
};
</script>
