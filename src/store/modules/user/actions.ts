import { ActionTree } from 'vuex'
import firebase from 'firebase/compat/app'
import 'firebase/compat/auth'
import { USER_STORE } from '@/store/constants'
import { IRootState, UserActionsTypes, UserState } from '@/store/interfaces'
import { Semester } from '@/store/classes/semester'

export const actions: ActionTree<UserState, IRootState> & UserActionsTypes = {
  [USER_STORE.ACTIONS.updateSemesterAsync]({ commit }) {
    const user = firebase.auth().currentUser
    if (user) {
      commit(USER_STORE.MUTATIONS.updateSemester)
    }
  },
  [USER_STORE.ACTIONS.addNewSemesterFromData]({ commit }, course_list) {
    commit(USER_STORE.MUTATIONS.addSemester, 0)
    for (const course of course_list) {
      commit(USER_STORE.MUTATIONS.addCourseWithDataToLastSemester, course)
    }
    commit(USER_STORE.MUTATIONS.setActiveSemester, -1)
  },
  [USER_STORE.ACTIONS.loadUserDataFromSite](
    { commit },
    data: {
      semesters: Semester[]
      english_exemption: boolean
      summer_semesters_indexes: number[]
    }
  ) {
    commit(USER_STORE.MUTATIONS.clearUserData)
    commit(USER_STORE.MUTATIONS.setSemesters, data.semesters)
    commit(USER_STORE.MUTATIONS.setExemptionStatus, data.english_exemption)
    commit(USER_STORE.MUTATIONS.reCalcCurrentSemester)
    const summer_semesters_indexes = data.summer_semesters_indexes
    for (const summerSemesterIndex of summer_semesters_indexes) {
      commit(USER_STORE.MUTATIONS.changeSemesterType, summerSemesterIndex)
    }
    commit(USER_STORE.MUTATIONS.setActiveSemester, -1)
  },
}
