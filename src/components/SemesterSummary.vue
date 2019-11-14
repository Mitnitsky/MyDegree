<template>

  <div class="container-fluid">
    <div class="col align-self-start"
         style="padding: 0 0;">
      <table class="table table-bordered">
        <thead>
        <tr>
          <th colspan="2"
              style="text-align: right"
              scope="col">סיכום סמסטר
          </th>
        </tr>
        </thead>
        <tbody>
          <tr>
            <td style="width:  30%; text-align: right;">
              <label style="margin: 8px 0px 8px 0px;  text-align: right;">ממוצע:</label>
            </td>
            <td style="width:  70%">
              <input v-model.number.lazy="this.$store.state.user.semesters[this.$store.state.user.active_semester].average"
                     @change="updateField('average')"
                     class="form-control"
                     max="100"
                     min="0"
                     readonly
                     style="text-align: center;width: 100%"
                     type="number">
            </td>
          </tr>
          <tr>
            <td style="width:  30%; text-align: right">
              <label style="margin: 8px 0px 8px 0px; ">נקודות:</label>
            </td>
            <td style="width:  70%">
              <input v-model.number.lazy="this.$store.state.user.semesters[this.$store.state.user.active_semester].points"
                     @change="updateField('points')"
                     class="form-control"
                     max="300"
                     min="0"
                     readonly
                     style="text-align: center;width: 100%"
                     type="number">
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script>
    export default {
        name: 'semester-summary',
        methods: {
            updateField(field) {
                let value = this.course[field];
                if(field)
                    this.$store.commit('updateSemesterSummary', {field, value, index: this.index});
                this.$store.commit('reCalcCurrentSemester');
                this.$store.dispatch('updateSemesterAsync');
            },
        },
    }
</script>