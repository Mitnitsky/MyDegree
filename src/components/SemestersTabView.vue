<template>
    <b-card no-body style="margin: 10px 20px">
        <b-tabs card pills vertical>
            <b-tab :key="semester.name" :title="'Semester '+ semester.name" v-for="semester in semesters">
                <div class="row">
                    <div class="col-lg-9">
                        <app-semester-table :semester="semester"/>
                    </div>
                    <div class="col-lg-3" style="padding: 0 0">
                        <app-semester-summary/>
                    </div>
                </div>
                <div class="row align-self-end">
                    <div class="col align-self-end">
                        <b-button @click="closeTab(semester.name)" class="float-right" size="sm" style="margin-bottom:10px"
                                  variant="danger">
                            Delete Semester
                        </b-button>
                    </div>
                </div>
            </b-tab>

            <!-- New Tab Button (Using tabs slot) -->
            <template slot="tabs-end">
                <b-nav-item @click.prevent="newTab" href="#"><b>+</b></b-nav-item>
            </template>

            <!-- Render this if no tabs -->
            <div class="container justify-content-md-center alert alert-secondary text-center text-muted" slot="empty">
                <h2>There are no semesters yet!</h2>
                <br>
                Create new semester using the <b>+</b> button on the left side.
            </div>
        </b-tabs>
    </b-card>
</template>


<script>
    import AppSemesterSummary from "@/components/SemesterSummary";
    import AppSemesterTable from "@/components/SemesterTable";

    export default {
        name: "semesters-tab-view",
        components: {AppSemesterTable, AppSemesterSummary},
        data() {
            return {
                semesters: [],
                tabCounter: 1
            }
        },
        methods: {
            closeTab(x) {
                if (confirm("Delete the semester?")) {
                    for (let i = 0; i < this.semesters.length; i++) {
                        if (this.semesters[i].name === x) {
                            this.semesters.splice(i, 1)
                        }
                    }
                    for (let i = 0; i < this.semesters.length; i++) {
                        this.semesters[i].name = i + 1;
                    }
                    this.tabCounter = this.semesters.length + 1;
                }
            },
            newTab() {
                this.semesters.push({name: this.tabCounter++, coursesNumber: 5})
            }
        }
    }
</script>

<style>
    .nav-link: {
        background-color: #2c3e50;
    }
</style>
