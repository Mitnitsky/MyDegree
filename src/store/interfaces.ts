import { Semester } from "@/store/classes/semester";
import { CourseType } from "@/store/classes/course_types";
import { ActionContext, DispatchOptions } from "vuex";
import { AUTH_STORE, USER_STORE } from "@/store/constants";
import { Course } from "@/store/classes/course";

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface IRootState {}

export interface IMergedState extends IRootState {
  authState: AuthState;
  userState: UserState;
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface IRootGettersTypes {}

// eslint-disable-next-line @typescript-eslint/ban-types
export type RootMutationsTypes<S = IRootState> = {};

type AugmentedActionContextRoot = {
  commit<K extends keyof RootMutationsTypes>(
    key: K,
    payload?: Parameters<RootMutationsTypes[K]>[1]
  ): ReturnType<RootMutationsTypes[K]>;
} & Omit<ActionContext<IRootState, IRootState>, "commit"> & {
    dispatch<K extends keyof StoreActions>(
      key: K,
      payload?: Parameters<StoreActions[K]>[1],
      options?: DispatchOptions
    ): ReturnType<StoreActions[K]>;
  };

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface IRootActionsTypes {}

/*********************** AUTH MODULE TYPES  ***********************/

export interface AuthState {
  logged: boolean;
  username: string;
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface AuthGettersTypes {}

export type AuthMutationsTypes<S = AuthState> = {
  [AUTH_STORE.MUTATIONS.setLoggedStatus](state: S, payload: boolean): void;
  [AUTH_STORE.MUTATIONS.setUserName](state: S, payload: string): void;
};

/**
 * probably this can be moved inside individual module
 * as counterTypes.ts and then can be imported here
 */
type AugmentedActionContextAuth = {
  commit<K extends keyof AuthMutationsTypes>(
    key: K,
    payload: Parameters<AuthMutationsTypes[K]>[1]
  ): ReturnType<AuthMutationsTypes[K]>;
} & Omit<ActionContext<AuthState, IRootState>, "commit">;

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface AuthActionsTypes {}

/*********************** COUNTER MODULE TYPES  ***********************/
export interface UserState {
  summer_semesters: number;
  active_semester: number;
  degree_average: number;
  degree_points: number;
  degree_points_done: number;
  degree_points_left: number;
  degree_points_to_choose: number;
  english_exemption: boolean;
  semesters: Semester[];
  course_types: CourseType[];
}

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface UserGettersTypes {}

export type UserMutationsTypes<S = UserState> = {
  [USER_STORE.MUTATIONS.clearUserData](state: S): void;
  [USER_STORE.MUTATIONS.setSemesters](state: S, semesters: Semester[]): void;
  [USER_STORE.MUTATIONS.setActiveSemester](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.setExemptionStatus](state: S, payload: boolean): void;
  [USER_STORE.MUTATIONS.addSemester](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.sortSemesterByField](
    state: S,
    payload: "number" | "grade" | "points" | "name"
  ): void;
  [USER_STORE.MUTATIONS.addCourse](state: S): void;
  [USER_STORE.MUTATIONS.addCourseWithData](state: S, payload: Course): void;
  [USER_STORE.MUTATIONS.addCourseWithDataToLastSemester](
    state: S,
    payload: Course
  ): void;
  [USER_STORE.MUTATIONS.addCourseWithDataReturningIndex](
    state: S,
    payload
  ): void;
  [USER_STORE.MUTATIONS.updateCourse](state: S, { field, value, index }): void;
  [USER_STORE.MUTATIONS.swapCourses](state: S, { a, b }): void;
  [USER_STORE.MUTATIONS.updateSemesterSummary](
    state: S,
    { field, value }
  ): void;
  [USER_STORE.MUTATIONS.updateInfo](state: S, { field, value }): void;
  [USER_STORE.MUTATIONS.removeCourse](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.moveCourse](state: S, { index, direction }): void;
  [USER_STORE.MUTATIONS.removeLastRow](state: S): void;
  [USER_STORE.MUTATIONS.removeSemester](state: S): void;
  [USER_STORE.MUTATIONS.changeSemesterTo](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.addCourseType](state: S, type: string): void;
  [USER_STORE.MUTATIONS.changeActiveSemesterType](state: S): void;
  [USER_STORE.MUTATIONS.changeSemesterType](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.clearCourse](state: S, index: number): void;
  [USER_STORE.MUTATIONS.changeCategoryName](state: S, name_index: any): void;
  [USER_STORE.MUTATIONS.deleteCourseType](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.moveCourseToSemester](
    state: S,
    { semester_index, course_index }
  ): void;
  [USER_STORE.MUTATIONS.reCalcCurrentSemester](state: S): void;
  [USER_STORE.MUTATIONS.updateSemester](state: S): void;
  [USER_STORE.MUTATIONS.exportSemesters](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.importCoursesFromJson](state: S, payload: string): void;
  [USER_STORE.MUTATIONS.fetchUserInfo](state: S, payload): void;
  [USER_STORE.MUTATIONS.checkIfCourseExists](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.checkPrerequisites](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.checkLinear](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.updateUserField](state: S, payload: number): void;
  [USER_STORE.MUTATIONS.updateSemesters](state: S, payload: number): void;
};

export type AugmentedActionContextUser = {
  commit<K extends keyof UserMutationsTypes>(
    key: K,
    payload?: Parameters<UserMutationsTypes[K]>[1]
  ): ReturnType<UserMutationsTypes[K]>;
} & Omit<ActionContext<UserState, IRootState>, "commit">;

export interface UserActionsTypes {
  [USER_STORE.ACTIONS.addNewSemesterFromData](
    { commit }: AugmentedActionContextUser,
    course_list
  ): void;

  [USER_STORE.ACTIONS.loadUserDataFromSite](
    { commit }: AugmentedActionContextUser,
    semesters_exemption_summerIndexes
  ): void;

  [USER_STORE.ACTIONS.updateSemesterAsync]({
    commit,
  }: AugmentedActionContextUser): void;
}

export interface StoreActions
  extends IRootActionsTypes,
    AuthActionsTypes,
    UserActionsTypes {}

export interface StoreGetters
  extends IRootGettersTypes,
    AuthGettersTypes,
    UserGettersTypes {}
