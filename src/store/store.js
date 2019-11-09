import Vue from 'vue';
import Vuex from 'vuex';
import {Semester} from '../semester'
Vue.use(Vuex);

export const store = new Vuex.Store({
    state: {
        logged: false,
        user: {
            token: '',
            name: '',
            active_semester: 0,
            semesters: [
            ],
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
        setLoginStatusTrue: (state) => {
            state.logged = true;
        },
        setLoginStatusFalse: (state) => {
            state.logged = false;
        },
        setUser: (state, user) => {
            if (user) {
                state.user.name = user.displayName;
                state.user.token = user.refreshToken;
            }
        },
        clearUserData: (state) => {
            state.user.name = '';
            state.user.picture = '';
            state.user.token = '';
        },
        addSemester: (state,initial_courses) => {
            state.user.semesters.push(new Semester(state.user.semesters.length+1, initial_courses));
        },
        addCourse: (state) => {
          state.user.semesters[state.user.active_semester].addCourse();
        },
        removeCourse: (state,index) => {
          state.user.semesters[state.user.active_semester].removeCourse(index);
        },
        removeLastRow: (state) => {
          state.user.semesters[state.user.active_semester].removeCourse(state.user.semesters.length-1);
        },
        removeSemester: (state) => {
            if (confirm("Delete the semester?")) {
                state.user.semesters.splice(state.user.active_semester, 1);
            }
            let i = 1;
            for(let semester of state.user.semesters){
                semester.name = i.toString();
                i++;
            }
        }
    },
    actions: {}
});
