<template>

        <div class="container">
            <div class="col align-self-start" style="padding: 0 0;">
                <table class="table table-bordered">
                    <thead>
                    <tr>
                        <th scope="col" colspan="2">Semester summary:</th>
                    </tr>
                    </thead>
                    <tbody>
                    <tr>
                        <td style="width:  30%">
                            <label style="margin: 8px 8px 0px 0px">Average:</label>
                        </td>
                        <td style="width:  70%">
                            <input type="number" class="form-control" max="100" min="0" value="0" readonly
                                   style="text-align: center;width: 100%">
                        </td>
                    </tr>
                    <tr>
                        <td style="width:  30%">
                            <label>Points:</label>
                        </td>
                        <td style="width:  70%">
                            <input type="number" class="form-control"
                                   max="300" min="0" value="0" readonly style="text-align: center;width: 100%">
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