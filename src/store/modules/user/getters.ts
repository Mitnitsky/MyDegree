import { GetterTree } from "vuex";
import { UserState, IRootState } from "@/store/interfaces";
import { USER_STORE } from "@/store/constants";

export const getters: GetterTree<UserState, IRootState> = {
  [USER_STORE.GETTERS.COURSE_TYPES]: (state: UserState) => {
    return state.course_types;
  },
};
