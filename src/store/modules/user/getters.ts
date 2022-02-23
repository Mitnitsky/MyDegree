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
};
