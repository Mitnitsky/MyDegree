import Vue from 'vue';
import Vuex from 'vuex';
import * as Semester from './classes/semester'
import firebase from "firebase";

Vue.use(Vuex);

export const store = new Vuex.Store({
    strict: true,
    state: {
        logged: false,
        user: {
            token: '',
            name: '',
            active_semester: 0,
            semesters: [],
        }
    },
    getters: {
        getLoginStatus: state => {
            return state.logged;
        },
        getUserName: state => {
            return state.user.name;
        },
        getUserToken: state => {
            return state.user.token;
        },
        getUserSemesters: state => {
            return state.user.semesters;
        }
    },
    mutations: {
        setLoginStatus: (state, status) => {
            state.logged = status;
        },
        setUser: (state, user) => {
            if (user) {
                state.user.name = user.displayName;
                state.user.token = user.refreshToken;
            }
        },
        clearUserData: (state) => {
            state.user.name = '';
            state.user.token = '';
            state.user.active_semester = 0;
            state.user.semesters = [];
        },
        addSemester: (state, initial_courses) =>   {
            state.user.semesters.push(Semester.createNewSemester(state.user.semesters.length + 1, initial_courses));
        },
        addCourse: (state) => {
            Semester.addCourseToSemester(state.user.semesters[state.user.active_semester]);
        },
        updateCourse: (state, {field, value, index}) => {
            Object.assign(state.user.semesters[state.user.active_semester].courses[index], {[field]:value});
        },
        removeCourse: (state, index) => {
            Semester.removeCourse(state.user.semesters[state.user.active_semester],index);
        },
        removeLastRow: (state) => {
            Semester.removeCourse(state.user.semesters[state.user.active_semester],state.user.semesters.length - 1);
        },
        removeSemester: (state) => {
            if (confirm("Delete the semester?")) {
                state.user.semesters.splice(state.user.active_semester, 1);
            }
            let i = 1;
            for (let semester of state.user.semesters) {
                semester.name = i.toString();
                i++;
            }
        },
        changeSemesterTo: (state,index) => {
            state.user.active_semester = index;
        },
        reCalcCurrentSemester: (state) => {
            window.console.log('Im calculating semeseter!');
            let  semester = state.user.semesters[state.user.active_semester]
            Semester.calculateAverage(semester);
            Semester.calculatePoints(semester);
        },
        updateSemester(state){
            const user = firebase.auth().currentUser;
            firebase.firestore().collection('users').doc(user.uid).update({
                        semesters: state.user.semesters
                    })
        }
    },
    actions: {
        updateSemesterAsync(context){
            const user = firebase.auth().currentUser;
            if(user) {
                context.commit('updateSemester');
            }
        }
    }
});
