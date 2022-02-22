import { ActionTree } from "vuex";
import firebase from "firebase/compat";
import { USER_STORE } from "@/store/constants";
import { IRootState, UserState, UserActionsTypes } from "@/store/interfaces";
import { Semester } from "@/store/classes/semester";

export const actions: ActionTree<UserState, IRootState> & UserActionsTypes = {
  [USER_STORE.ACTIONS.updateSemesterAsync]({ commit }) {
    const user = firebase.auth().currentUser;
    if (user) {
      commit(USER_STORE.MUTATIONS.updateSemester);
    }
  },
  [USER_STORE.ACTIONS.addNewSemesterFromData]({ commit }, course_list) {
    commit(USER_STORE.MUTATIONS.addSemester, 0);
    for (const course of course_list) {
      commit(USER_STORE.MUTATIONS.addCourseWithDataToLastSemester, course);
    }
    commit(USER_STORE.MUTATIONS.setActiveSemester, -1);
  },
  [USER_STORE.ACTIONS.loadUserDataFromSite](
    { commit },
    semesters_exemption_summerIndexes: {
      semesters: Semester[];
      exemption: boolean;
      summer_semesters_indexes: number[];
    }
  ) {
    commit(USER_STORE.MUTATIONS.clearUserData);
    commit(
      USER_STORE.MUTATIONS.setSemesters,
      semesters_exemption_summerIndexes.semesters
    );
    commit(
      USER_STORE.MUTATIONS.setExemptionStatus,
      semesters_exemption_summerIndexes.exemption
    );
    commit(USER_STORE.MUTATIONS.reCalcCurrentSemester);
    const summer_semesters_indexes =
      semesters_exemption_summerIndexes.summer_semesters_indexes;
    for (let i = 0; i < summer_semesters_indexes.length; i++) {
      commit(
        USER_STORE.MUTATIONS.changeSemesterType,
        summer_semesters_indexes[i]
      );
    }
    commit(USER_STORE.MUTATIONS.setActiveSemester, -1);
  },
};
