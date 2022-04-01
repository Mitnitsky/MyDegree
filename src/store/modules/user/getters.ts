import { GetterTree } from "vuex";
import { UserState, IRootState } from "@/store/interfaces";
import { USER_STORE } from "@/store/constants";

export const getters: GetterTree<UserState, IRootState> = {
  [USER_STORE.GETTERS.COURSE_TYPES]: (state: UserState) => {
    return state.course_types;
  },
  [USER_STORE.GETTERS.ACTIVE_SEMESTER]: (state: UserState) => {
    return state.active_semester;
  },
  [USER_STORE.GETTERS.SEMESTERS]: (state: UserState) => {
    return state.semesters;
  },
  [USER_STORE.GETTERS.DEGREE_AVERAGE]: (state: UserState) => {
    return state.degree_average;
  },
  [USER_STORE.GETTERS.DEGREE_POINTS]: (state: UserState) => {
    return state.degree_points;
  },
  [USER_STORE.GETTERS.DEGREE_POINTS_DONE]: (state: UserState) => {
    return state.degree_points_done;
  },
  [USER_STORE.GETTERS.DEGREE_POINTS_LEFT]: (state: UserState) => {
    return state.degree_points_left;
  },
  [USER_STORE.GETTERS.DEGREE_POINTS_TO_CHOOSE]: (state: UserState) => {
    return state.degree_points_to_choose;
  },
  [USER_STORE.GETTERS.ENGLISH_EXEMPTION]: (state: UserState) => {
    return state.english_exemption;
  },
};
