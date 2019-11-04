<template>
    <div class="container justify-content-md-center alert alert-secondary text-center text-muted" style="margin-bottom: 60px">

        <div class="row">
        <div class="col-sm-6">
            <h3 style="text-decoration: underline; ">Degree summary</h3>
            <div class="input-group mb-3" style="margin-top: 62px">
                <div class="input-group-prepend categoryName">
                    <span class="input-group-text categoryNameSpan"  >Degree's points</span>
                </div>
                <input type="number" class="form-control degree-summary" max="9999999" min="0" step="1" style="text-align: center" >
            </div>
            <div class="input-group mb-3">
                <div class="input-group-prepend categoryName">
                    <span class="input-group-text categoryNameSpan" >Average</span>
                </div>
                <input type="text" class="form-control degree-summary" readonly disabled="disabled">
            </div>
            <div class="input-group mb-3" >
                <div class="input-group-prepend categoryName">
                    <span class="input-group-text categoryNameSpan" >Points done</span>
                </div>
                <input type="text" class="form-control"  disabled="disabled"  readonly>
                <div class="input-group-append degree-summary degree-summary">

                </div>
            </div>
            <div class="input-group mb-3" >
                <div class="input-group-prepend categoryName">
                    <span class="input-group-text categoryNameSpan" >Points left</span>
                </div>
                <input type="text" class="form-control"  disabled="disabled"  readonly>
                <div class="input-group-append degree-summary degree-summary">

                </div>
            </div>
            <div class="input-group mb-3" >
                <div class="input-group-prepend categoryName">
                    <span class="input-group-text categoryNameSpan" >Not chosen</span>
                </div>
                <input type="text" class="form-control"  disabled="disabled"  readonly>
                <div class="input-group-append degree-summary degree-summary">

                </div>
            </div>

        </div>
            <div class="col-sm-6">
                <h3 style="text-decoration: underline; ">Courses summary</h3>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">

                    </div>
                    <input type="text" class="form-control" style="background-color: aliceblue"  readonly value="Left:">
                    <input type="text" class="form-control" style="background-color: aliceblue"  readonly value="Required:">
                </div>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">Mandatory: </span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">A' List</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">B' List</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">Humanities</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">
                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">Free choice:</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">

                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan">Projects:</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">

                    <div class="input-group-prepend courseName">
                        <span class="input-group-text courseNameSpan" >Sport:</span>
                    </div>
                    <input type="text" class="form-control"  disabled="disabled"  readonly>
                    <input type="number" class="form-control"  max="9999999" min="0" step="1" style="text-align: center">
                </div>
                <div class="input-group mb-3">
                    <b-form-checkbox
                            id="checkbox-1"
                            v-model="englishExemption"
                            name="checkbox-1"
                    >
                        English's Exemption
                    </b-form-checkbox>
                </div>
            </div>
        </div>
    </div>
</template>
<script>
    export default {
        name: 'degree-summary',
        data () {
            return {
                englishExemption: false
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
    .courseName{
        width: 30%;
    }
    .courseNameSpan{
        width: 100%;
        background-color: aliceblue;
    }
    .left{

    }
    .toDo{

    }
    .degree-summary{
        margin-right: 50px;
    }
    .categoryName{
        width: 40%;
    }
    .categoryNameSpan{
        width: 100%;
        background-color: aliceblue;
    }

</style>