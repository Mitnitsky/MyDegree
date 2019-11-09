<template>
    <b-card no-body>
        <b-tabs card>
            <!-- Render Tabs, supply a unique `key` to each tab -->
            <b-tab :key="'dyn-tab-' + i" :title="'Semester ' + (i+1)" v-for="i in 2">
                <div class="col-10">
                    <table class="table table-bordered">
                        <thead>
                        <tr>
                            <th scope="col">Type</th>
                            <th scope="col">Course Num</th>
                            <th scope="col">Name</th>
                            <th scope="col">Points</th>
                            <th scope="col">Grade</th>
                            <th scope="col" style="width:30px"></th>
                        </tr>
                        </thead>
                        <tbody>
                        <tr>
                            <td>
                                <select class="form-control">
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
                            <td><input class="form-control" max="9999999" min="0" step="1" type="number"></td>
                            <td><input class="form-control" type="text"></td>
                            <td><input class="form-control" max="25" min="0" step="0.5" type="number"
                                       value="0.0"></td>
                            <td><input class="form-control" max="100" min="0" step="1" type="number" value="0">
                            </td>
                            <td>
                                <b-button squared title="Remove Line" v-b-tooltip.hover
                                          variant="outline-danger">x
                                </b-button>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <select class="form-control">
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
                            <td><input class="form-control" max="9999999" min="0" step="1" type="number"></td>
                            <td><input class="form-control" type="text"></td>
                            <td><input class="form-control" max="25" min="0" step="0.5" type="number"
                                       value="0.0"></td>
                            <td><input class="form-control" max="100" min="0" step="1" type="number" value="0">
                            </td>
                            <td>
                                <b-button squared title="Remove Line" v-b-tooltip.hover
                                          variant="outline-danger">x
                                </b-button>
                            </td>
                        </tr>
                        </tbody>
                    </table>
                    <div class="row justify-content-md-center">
                        <b-button-group class="mx-1">
                            <b-button squared variant="outline-primary">Add Row</b-button>
                            <b-button squared variant="outline-danger">Remove Row</b-button>
                        </b-button-group>
                    </div>
                    <b-button @click="closeTab(i)" class="float-right" size="sm" style="margin-bottom:10px"
                              variant="danger">
                        Delete Semester
                    </b-button>
                </div>
                <div class="col-3">
                    <table class="table table-sm table-light">
                        <thead>
                        <tr>
                            <th colspan="2" scope="col">Semester summary:</th>
                        </tr>
                        </thead>
                        <tbody>
                        <tr>
                            <td>
                                <label>Average:</label>
                            </td>
                            <td>
                                <input class="form-control" max="100" min="0" readonly style="width:40px" type="number"
                                       value="0">
                            </td class="w-25">
                        </tr>
                        <tr>
                            <td>
                                <label>Points:</label>
                            </td>
                            <td>
                                <input aria-describedby="basic-addon1" class="form-control" max="300"
                                       min="0" readonly style="width:40px" type="number" value="0">
                            </td>
                        </tr>
                        </tbody>
                    </table>
                </div>
            </b-tab>

            <!-- New Tab Button (Using tabs slot) -->
            <template slot="tabs-end">
                <b-nav-item @click.prevent="newTab" href="#"><b>+</b></b-nav-item>
            </template>

            <!-- Render this if no tabs -->
            <div class="text-center text-muted" slot="empty">
                There are no open tabs<br>
                Open a new tab using the <b>+</b> button above.
            </div>
        </b-tabs>
    </b-card>
</template>

<script>
    export default {
        data() {
            return {
                tabs: [],
                tabCounter: 0
            }
        },
        methods: {
            closeTab(x) {
                for (let i = 0; i < this.tabs.length; i++) {
                    if (this.tabs[i] === x) {
                        this.tabs.splice(i, 1)
                    }
                }
            },
            newTab() {
                this.tabs.push(this.tabCounter++)
            }
        }
    }
</script>

<style scoped>

</style>