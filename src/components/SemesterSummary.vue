<template>

  <div class="container">
    <div class="col align-self-start"
         style="padding: 0 0;">
      <table class="table table-bordered">
        <thead>
        <tr>
          <th colspan="2"
              scope="col">סיכום סמסטר
          </th>
        </tr>
        </thead>
        <tbody>
          <tr>
            <td style="width:  30%">
              <label style="margin: 8px 8px 0px 0px">ממוצע:</label>
            </td>
            <td style="width:  70%">
              <input class="form-control"
                     max="100"
                     min="0"
                     readonly
                     style="text-align: center;width: 100%"
                     type="number"
                     :value="this.$store.state.user.semesters[this.$store.state.user.active_semester].average">
            </td>
          </tr>
          <tr>
            <td style="width:  30%">
              <label>נקודות:</label>
            </td>
            <td style="width:  70%">
              <input class="form-control"
                     max="300"
                     min="0"
                     readonly
                     style="text-align: center;width: 100%"
                     type="number"
                     :value="this.$store.state.user.semesters[this.$store.state.user.active_semester].points">
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
<script>
    export default {
        name: 'semester-summary'
    }
</script>