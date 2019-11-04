<template>
    <tr>
        <td class="courseType">
            <select class="form-control" style="text-align-last: center;">
                <option value="Mandatory">Mandatory</option>
                <option value="A' List">A' List</option>
                <option value="B' List">B' List</option>
                <option value="Humanistic">Humanistic</option>
                <option value="Free Choice">Free Choice</option>
                <option value="Project">Project</option>
                <option value="Sport">Sport</option>
                <option value="Exemption">Exemption</option>
            </select>
        </td>
        <td class="courseNumber">
            <input class="form-control" max="9999999" min="0" step="1" style="text-align: center" type="number">
        </td>
        <td class="courseName"><input class="form-control" style="text-align: center" type="text"></td>
        <td class="coursePoints"><input class="form-control" max="25" min="0" step="0.5" style="text-align: center"
                                      type="number" value="0.0"></td>
        <td class="courseGrade"><input class="form-control" max="100" min="0" step="1" style="text-align: center"
                                      type="number" value="0"></td>
        <td class="clearButton text-center">
            <b-button  squared title="Remove Row" v-b-tooltip.hover
                      variant="outline-danger">x
            </b-button>
        </td>
    </tr>
</template>
<script>
    export default {
        name: 'semester-table-course-row'
    }
</script>

<style scoped>
    input[type=number]::-webkit-inner-spin-button,
    input[type=number]::-webkit-outer-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }
    .courseType{

    }
    .courseNumber{

    }
    .courseName{

    }
    .courseGrade{

    }
    .coursePoints{

    }
    .clearButton{

    }
</style>
