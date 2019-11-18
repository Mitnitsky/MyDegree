import Vue from 'vue';
import Vuex from 'vuex';
import * as Semester from './classes/semester'
import * as Course from './classes/course'
import firebase from "firebase";
import {getField, updateField} from 'vuex-map-fields';
import {courseExistInSemesters} from "./classes/semester";

Vue.use(Vuex);

export const store = new Vuex.Store({
    state: {
        logged: false,
        user_name: '',
        user: {
            active_semester: 0,
            degree_average: 0,
            degree_points: 0,
            degree_points_done: 0,
            degree_points_left: 0,
            degree_points_to_choose: 0,
            must_points: 0,
            must_points_left: 0,
            a_list_points: 0,
            a_list_points_left: 0,
            b_list_points: 0,
            b_list_points_left: 0,
            humanistic_points: 0,
            humanistic_points_left: 0,
            free_points: 0,
            free_points_left: 0,
            projects_points: 0,
            projects_points_left: 0,
            sport: 0,
            sport_left: 0,
            exemption_points: 0,
            english_exemption: false,
            semesters: [],
        }
    },
    getters: {
        getField,
        getUserField(state) {
            return getField(state.user);
        },
    },
    mutations: {
        updateField,
        updateDegreePoints(state, points) {
            state.user.degree_points = points;
        },
        setLoginStatus: (state, status) => {
            state.logged = status;
        },
        setUser: (state, user) => {
            if (user) {
                state.user_name = user.displayName;
            }
        },
        clearUserData: (state) => {
            state.user.active_semester = 0;
            state.user.degree_average = 0;
            state.user.degree_points = 0;
            state.user.degree_points_done = 0;
            state.user.degree_points_left = 0;
            state.user.degree_points_to_choose = 0;
            state.user.must_points = 0;
            state.user.must_points_left = 0;
            state.user.a_list_points = 0;
            state.user.a_list_points_left = 0;
            state.user.b_list_points = 0;
            state.user.b_list_points_left = 0;
            state.user.humanistic_points = 0;
            state.user.humanistic_points_left = 0;
            state.user.free_points = 0;
            state.user.free_points_left = 0;
            state.user.projects_points = 0;
            state.user.projects_points_left = 0;
            state.user.sport = 0;
            state.user.sport_left = 0;
            state.user.exemption_points = 0;
            state.user.english_exemption = false;
            state.user.semesters = [];
        },
        addSemester: (state, initial_courses) => {
            state.user.semesters.push(Semester.createNewSemester(state.user.semesters.length + 1, initial_courses));
        },
        addCourse: (state) => {
            Semester.addCourseToSemester(state.user.semesters[state.user.active_semester]);
        },
        addCourseWithData: (state, course) => {
            Semester.addExistingCourse(state.user.semesters[state.user.active_semester], course);
        },
        updateCourse: (state, {field, value, index}) => {
            Object.assign(state.user.semesters[state.user.active_semester].courses[index], {[field]: value});
        },
        updateSemesterSummary: (state, {field, value}) => {
            Object.assign(state.user.semesters[state.user.active_semester], {[field]: value});
        },
        updateInfo: (state, {field, value}) => {
            Object.assign(state.user, {[field]: value});
        },
        removeCourse: (state, index) => {
            Semester.removeCourse(state.user.semesters[state.user.active_semester], index);
        },
        removeLastRow: (state) => {
            let current_semester = state.user.semesters[state.user.active_semester];
            let last_course_index = current_semester.courses.length - 1;
            if (!Course.courseIsEmpty(current_semester.courses[last_course_index])) {
                if (confirm("למחוק קורס בעל תוכן?"))
                    Semester.removeCourse(current_semester, last_course_index);
            }else{
                Semester.removeCourse(current_semester, last_course_index);
            }
        },
        removeSemester: (state) => {
            if (confirm("למחוק סמסטר זה?")) {
                state.user.semesters.splice(state.user.active_semester, 1);
            }
            let i = 1;
            for (let semester of state.user.semesters) {
                semester.name = i.toString();
                i++;
            }
        },
        changeSemesterTo: (state, index) => {
            state.user.active_semester = index;
        },
        reCalcCurrentSemester: (state) => {
            let current_semester = state.user.semesters[state.user.active_semester];
            Semester.calculateAverage(current_semester);
            Semester.calculatePoints(current_semester);
            if (state.user.english_exemption) {
                state.user.degree_points_done = 3;
                state.must_points = 0;
            } else {
                state.user.degree_points_done = 0;
            }
            state.user.degree_average = 0;
            state.user.degree_points_to_choose = state.user.degree_points;
            state.user.must_points_left = state.user.must_points;
            state.user.a_list_points_left = state.user.a_list_points;
            state.user.b_list_points_left = state.user.b_list_points;
            state.user.humanistic_points_left = state.user.humanistic_points;
            state.user.free_points_left = state.user.free_points;
            state.user.projects_points_left = state.user.projects_points;
            state.user.sport_left = state.user.sport;
            for (const semester of state.user.semesters) {
                state.user.degree_average += semester.points_done * semester.average;
                state.user.degree_points_done += semester.points_done;
                state.user.degree_points_to_choose -= semester.points;
                state.user.must_points_left -= semester.must_points;
                state.user.a_list_points_left -= semester.a_list_points;
                state.user.b_list_points_left -= semester.b_list_points;
                state.user.humanistic_points_left -= semester.humanistic_points;
                state.user.free_points_left -= semester.free_points;
                state.user.projects_points_left -= semester.projects_points;
                state.user.sport_left -= semester.sport;
            }
            state.user.degree_average /= state.user.degree_points_done;
            state.user.degree_points_left = state.user.degree_points - state.user.degree_points_done;
            if (localStorage.getItem('authenticated') === 'true') {
                const user = firebase.auth().currentUser;
                if(user != null){
                    firebase.firestore().collection('users').doc(user.uid).set(state.user).then((result) => {
                        return typeof result;
                    }).catch((reason => {
                        window.console.log('Error uploading user-data (' + reason + ')');
                    }));
                }
            } else {

                localStorage.setItem('saved_session_data', JSON.stringify(state.user));
                localStorage.setItem('authenticated', 'false');
            }
        },
        updateSemester(state) {
            const user = firebase.auth().currentUser;
            if (user != null) {
                firebase.firestore().collection('users').doc(user.uid).update({
                    semesters: state.user.semesters
                })
            }
        },
        fetchUserInfo: (state, user) => {
            state.user = user;
        },
        checkIfCourseExists: (state, course_number_and_answer) => {
            course_number_and_answer['answer'] = courseExistInSemesters(state.user.semesters, course_number_and_answer.course_number);
        },
        checkPrerequisites: (state, course_number_and_answer) => {
            course_number_and_answer['answer'] = courseExistInSemesters(state.user.semesters, course_number_and_answer.course_number, state.user.active_semester - 1);
        },
        checkLinear: (state, course_number_and_answer) => {
            course_number_and_answer['answer'] = courseExistInSemesters(state.user.semesters, course_number_and_answer.course_number, state.user.active_semester);
        },
        updateUserField(state, field) {
            updateField(state.user, field);
            //TODO: handle the copy paste here
            let current_semester = state.user.semesters[state.user.active_semester];
            Semester.calculateAverage(current_semester);
            Semester.calculatePoints(current_semester);
            if (state.user.english_exemption) {
                state.user.degree_points_done = 3;
                state.must_points = 0;
            } else {
                state.user.degree_points_done = 0;
            }
            state.user.degree_average = 0;
            state.user.degree_points_to_choose = state.user.degree_points;
            state.user.must_points_left = state.user.must_points;
            state.user.a_list_points_left = state.user.a_list_points;
            state.user.b_list_points_left = state.user.b_list_points;
            state.user.humanistic_points_left = state.user.humanistic_points;
            state.user.free_points_left = state.user.free_points;
            state.user.projects_points_left = state.user.projects_points;
            state.user.sport_left = state.user.sport;
            for (const semester of state.user.semesters) {
                state.user.degree_average += semester.points_done * semester.average;
                state.user.degree_points_done += semester.points_done;
                state.user.degree_points_to_choose -= semester.points;
                state.user.must_points_left -= semester.must_points;
                state.user.a_list_points_left -= semester.a_list_points;
                state.user.b_list_points_left -= semester.b_list_points;
                state.user.humanistic_points_left -= semester.humanistic_points;
                state.user.free_points_left -= semester.free_points;
                state.user.projects_points_left -= semester.projects_points;
                state.user.sport_left -= semester.sport;
            }
            state.user.degree_average /= state.user.degree_points_done;
            state.user.degree_points_left = state.user.degree_points - state.user.degree_points_done;
            if (localStorage.getItem('authenticated') === 'true') {
                const user = firebase.auth().currentUser;
                if(user != null){
                    firebase.firestore().collection('users').doc(user.uid).set(state.user).then((result) => {
                        return typeof result;
                    }).catch((reason => {
                        window.console.log('Error uploading user-data (' + reason + ')');
                    }));
                }
            } else {

                localStorage.setItem('saved_session_data', JSON.stringify(state.user));
                localStorage.setItem('authenticated', 'false');
            }
        },
    },
    actions: {
        updateSemesterAsync(context) {
            const user = firebase.auth().currentUser;
            if (user) {
                context.commit('updateSemester');
            }

        }
    }
});
