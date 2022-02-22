export enum UserGetters {
  COURSE_TYPES = "COURSE_TYPES",
  SEMESTERS = "SEMESTERS",
}

export enum UserMutations {
  swapCourses = "swapCourses",
  clearUserData = "clearUserData",
  setSemesters = "setSemseters",
  setActiveSemester = "setActiveSemester",
  setExemptionStatus = "setExemptionStatus",
  addSemester = "addSemester",
  sortSemesterByField = "sortSemesterByField",
  addCourse = "addCourse",
  addCourseWithData = "addCourseWithData",
  addCourseWithDataToLastSemester = "addCourseWithDataToLastSemester",
  addCourseWithDataReturningIndex = "addCourseWithDataReturningIndex",
  updateCourse = "updateCourse",
  updateSemesterSummary = "updateSemesterSummary",
  updateInfo = "updateInfo",
  removeCourse = "removeCourse",
  moveCourse = "moveCourse",
  removeLastRow = "removeLastRow",
  removeSemester = "removeSemester",
  changeSemesterTo = "changeSemesterTo",
  addCourseType = "addCourseType",
  changeActiveSemesterType = "changeActiveSemesterType",
  changeSemesterType = "changeSemesterType",
  changeCategoryName = "changeCategoryName",
  deleteCourseType = "deleteCourseType",
  moveCourseToSemester = "moveCourseToSemester",
  reCalcCurrentSemester = "reCalcCurrentSemester",
  updateSemester = "updateSemester",
  exportSemesters = "exportSemesters",
  importCoursesFromJson = "importCoursesFromJson",
  fetchUserInfo = "fetchUserInfo",
  checkIfCourseExists = "checkIfCourseExists",
  checkPrerequisites = "checkPrerequisites",
  checkLinear = "checkLinear",
  updateUserField = "updateUserField",
  updateSemesters = "updateSemesters",
  clearCourse = "clearCourse",
}

export enum UserActions {
  updateSemesterAsync = "updateSemesterAsync",
  addNewSemesterFromData = "addNewSemesterFromData",
  loadUserDataFromSite = "loadUserDataFromSite",
}

export enum AuthGetters {
  USERNAME = "USERNAME",
  LOGGED = "LOGGED",
}

export enum AuthMutations {
  setUserName = "setUserName",
  setLoggedStatus = "setLoggedStatus",
}

export enum AuthActions {}
