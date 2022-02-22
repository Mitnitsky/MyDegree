import { MutationTree } from "vuex";
import { UserState } from "@/store/interfaces";
import {
  courseExistInSemesters,
  getSummerSemestersNumber,
  renameSemesters,
  Semester,
} from "@/store/classes/semester";
import { MathRound10 } from "@/store/extensions/rounder";
import { findCourse } from "@/store/extensions/converter";
import {
  create_course_type,
  default_course_types_obj,
} from "@/store/classes/course_types";
// v9 compat packages are API compatible with v8 code
import { db, auth } from "@/main";
import { stateConverter } from "@/firestore/firestoreconverter";
import { saveJSON } from "@/store/extensions/download";
import { JsonCourseDB } from "@/store/classes/json_course_db";
import { Course } from "@/store/classes/course";
import { USER_STORE } from "@/store/constants";

let jsonCourses: JsonCourseDB;
const courses = localStorage.getItem("courses");
if (courses) {
  jsonCourses =
    typeof localStorage.getItem("courses") === "object"
      ? localStorage.getItem("courses")
      : JSON.parse(courses);
  if (!jsonCourses.version || jsonCourses.version < 7.0) {
    jsonCourses = require("../../../data/courses.json");
    localStorage.setItem("courses", JSON.stringify(jsonCourses));
  }
} else {
  jsonCourses = require("../../../data/courses.json");
  localStorage.setItem("courses", JSON.stringify(jsonCourses));
}

function updateUserData(state: UserState) {
  if (localStorage.getItem("authenticated") === "true") {
    const user = auth.currentUser;
    if (user != null) {
      const users = db.collection("users").withConverter(stateConverter);
      const doc = users.doc(user.uid);
      doc.set(state).catch((reason) => {
        console.log("Error uploading user-data (" + reason + ")");
      });
    }
  } else {
    localStorage.setItem("saved_session_data", JSON.stringify(state));
    localStorage.setItem("authenticated", "false");
  }
}

function calculateUserInfo(state: UserState): void {
  const current_semester = state.semesters[state.active_semester];
  const exemption_index = 1;
  const mandatory_index = 0;
  const english_exemption_points = state.english_exemption ? 3 : 0;

  let exemption_points = 0;
  let failed_points = 0;
  let binary_points = 0;
  if (current_semester != null) {
    state.degree_points_done = english_exemption_points;
    state.degree_average = 0;
    state.degree_points_to_choose =
      state.degree_points - state.degree_points_done;
    state.degree_points_left = state.degree_points - state.degree_points_done;
    state.course_types[mandatory_index].points_left =
      state.course_types[mandatory_index].points_required -
      english_exemption_points;
    state.course_types[exemption_index].points_left = english_exemption_points;
    for (const course_type of state.course_types) {
      course_type.average = 0.0;
      course_type.points_done = 0.0;
      course_type.total_points = 0;
      if (!(course_type.name === "חובה" || course_type.name === "פטור")) {
        course_type.points_left = course_type.points_required;
      }
      if (course_type.name === "פטור") {
        course_type.total_points = english_exemption_points;
      }
    }
    const courses_done = {};
    for (const semester of state.semesters.slice().reverse()) {
      semester.calculateAverage();
      semester.calculatePoints();
      for (const course of semester.courses) {
        const course_has_number = course.number.toString().length > 2;
        const course_already_done =
          course.name in courses_done && course_has_number;
        const number_index = 0;
        const grade_index = 1;
        const binary_index = 2;
        if (
          course.name.includes("ספורט") ||
          course.name.includes("גופני") ||
          !(
            course_already_done &&
            course.number === courses_done[course.name][number_index] &&
            (parseInt(courses_done[course.name][grade_index]) !== 0 ||
              courses_done[course.name][binary_index])
          )
        ) {
          const course_points = course.points;
          if (
            course.name.includes("ספורט") ||
            course.name.includes("גופני") ||
            !(
              course_already_done &&
              course.number === courses_done[course.name][number_index]
            )
          ) {
            state.course_types[course.type].total_points += course_points;
            if (!state.course_types[course.type].name.includes("פטור")) {
              state.course_types[course.type].points_left -= course_points;
            }
            state.degree_points_to_choose -= course_points;
          }
          const course_grade = course.grade;
          if (
            ((course.binary || course_grade > 0) && !course_already_done) ||
            (!course_already_done && course.type === exemption_index) ||
            ((course.binary || course_grade > 0) &&
              (course.name.includes("ספורט") ||
                course.name.includes("גופני"))) ||
            (course_already_done &&
              (parseInt(courses_done[course.name][grade_index]) === 0 ||
                courses_done[course.name][binary_index]))
          ) {
            if (
              course.type !== exemption_index &&
              !(course.binary !== undefined && course.binary)
            ) {
              state.degree_average += course_points * course_grade;
              state.course_types[course.type].average +=
                course_points * course_grade;
              state.course_types[course.type].points_done += course_points;
            }
            state.degree_points_left -= course_points;
            if (
              course_grade >= 55 ||
              course.type === exemption_index ||
              course.binary
            ) {
              if (course.type === exemption_index) {
                exemption_points += course_points;
              }
              if (course.binary && course.type !== exemption_index) {
                binary_points += course_points;
              }

              state.degree_points_done += course_points;
            } else if (course_grade !== 0) {
              failed_points += course_points;
            }
          }
          const course_info = findCourse(course.number, jsonCourses.courses);

          courses_done[course.name] = [
            course.number,
            course.grade,
            course.binary,
          ];
          if (course_info.length > 0) {
            const first_course_info = course_info[0];
            for (const overlappingKey of first_course_info.overlapping) {
              const fullname = overlappingKey.split(":");
              const course_number = fullname[0];
              const course_name = fullname.slice(1).join().trim();
              courses_done[course_name] = [course_number, course_grade];
            }
            for (const identicalKey of first_course_info.identical) {
              const fullname = identicalKey.split(":");
              const course_number = fullname[0];
              const course_name = fullname.slice(1).join().trim();
              courses_done[course_name] = [course_number, course_grade];
            }
            for (const inclusiveKey of first_course_info.inclusive) {
              const fullname = inclusiveKey.split(":");
              const course_number = fullname[0];
              const course_name = fullname.slice(1).join().trim();
              courses_done[course_name] = [course_number, course_grade];
            }
          }
        }
      }
    }
    const degree_points_with_grade =
      state.degree_points_done -
      english_exemption_points -
      exemption_points -
      binary_points +
      failed_points;
    if (degree_points_with_grade !== 0) {
      state.degree_average /= degree_points_with_grade;
      state.degree_average = MathRound10(state.degree_average, -1).toFixed(1);
    } else {
      state.degree_average = 0;
    }
    for (const course_type of state.course_types) {
      if (course_type.points_done > 0) {
        course_type.average /= course_type.points_done;
        course_type.average = MathRound10(course_type.average, -1).toFixed(1);
      }
    }
    state.degree_points_left = state.degree_points - state.degree_points_done;
  }
  updateUserData(state);
}

function resetRemovedCategory(state: UserState, category_id: number) {
  for (const semester of state.semesters) {
    for (const course of semester.courses) {
      if (course.type === category_id) {
        course.type = 0;
      } else {
        if (course.type > category_id) {
          course.type -= 1;
        }
      }
    }
  }
}

export const mutations: MutationTree<UserState> = {
  [USER_STORE.MUTATIONS.clearUserData](state) {
    state.active_semester = 0;
    state.summer_semesters = 0;
    state.degree_average = 0;
    state.degree_points = 0;
    state.degree_points_done = 0;
    state.degree_points_left = 0;
    state.degree_points_to_choose = 0;
    state.english_exemption = false;
    state.semesters = [];
    state.course_types = default_course_types_obj;
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.setSemesters](state: UserState, semesters: Semester[]) {
    state.semesters = semesters;
  },
  [USER_STORE.MUTATIONS.setActiveSemester](state: UserState, index: number) {
    if (index === -1) {
      setTimeout(() => {
        state.active_semester = state.semesters.length - 1;
      }, 500);
    } else {
      state.active_semester = index;
    }
  },
  [USER_STORE.MUTATIONS.setExemptionStatus](state: UserState, status: boolean) {
    state.english_exemption = status;
  },
  [USER_STORE.MUTATIONS.addSemester](
    state: UserState,
    initial_courses: number
  ) {
    state.summer_semesters = getSummerSemestersNumber(state.semesters);
    state.semesters.push(
      new Semester(String(state.semesters.length + 1), initial_courses)
    );
    renameSemesters(state.semesters);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.sortSemesterByField](state: UserState, fieldName) {
    state.semesters[state.active_semester].sortCoursesByField(fieldName);
  },
  [USER_STORE.MUTATIONS.addCourse](state) {
    state.semesters[state.active_semester].addCourse();
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.addCourseWithData](state: UserState, course: Course) {
    state.semesters[state.active_semester].addExistingCourseReturnIndex(course);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.addCourseWithDataToLastSemester](
    state: UserState,
    course
  ) {
    state.semesters[state.semesters.length - 1].addExistingCourseReturnIndex(
      course
    );
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.addCourseWithDataReturningIndex](
    state: UserState,
    course_and_return_index
  ) {
    course_and_return_index["added_index"] = state.semesters[
      state.active_semester
    ].addExistingCourseReturnIndex(course_and_return_index["course"]);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.updateCourse](
    state: UserState,
    { field, value, index }
  ) {
    Object.assign(state.semesters[state.active_semester].courses[index], {
      [field]: value,
    });
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.updateSemesterSummary](
    state: UserState,
    { field, value }
  ) {
    Object.assign(state.semesters[state.active_semester], {
      [field]: value,
    });
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.updateInfo](state: UserState, { field, value }) {
    Object.assign(state, { [field]: value });
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.removeCourse](state: UserState, index) {
    state.semesters[state.active_semester].removeCourse(index);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.moveCourse](state: UserState, { index, direction }) {
    const active_semester = state.active_semester;
    const current_semester = state.semesters[active_semester];
    const direction_int = direction === "up" ? -1 : 1;
    if (
      !(
        (current_semester.courses.length - 1 === index &&
          direction === "down") ||
        (index === 0 && direction === "up")
      )
    ) {
      const temp = current_semester.courses[index];
      current_semester.courses[index] =
        current_semester.courses[index + direction_int];
      current_semester.courses[index + direction_int] = temp;
    }
  },
  [USER_STORE.MUTATIONS.removeLastRow](state) {
    const current_semester = state.semesters[state.active_semester];
    const last_course_index = current_semester.courses.length - 1;
    current_semester.removeCourse(last_course_index);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.removeSemester](state) {
    const semesters = state.semesters;
    if (semesters.length === 1) {
      state.semesters = [];
      state.summer_semesters = 0;
      return;
    }
    semesters.splice(state.active_semester, 1);
    renameSemesters(semesters);
    state.summer_semesters = getSummerSemestersNumber(semesters);
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.changeSemesterTo](state: UserState, index) {
    state.active_semester = index;
    updateUserData(state);
  },
  [USER_STORE.MUTATIONS.addCourseType](state: UserState, typeName: string) {
    if (typeName.toString() !== "") {
      for (const type of state.course_types) {
        if (type.name === typeName.toString()) {
          return;
        }
      }
      state.course_types.push(create_course_type(typeName));
      updateUserData(state);
    }
  },
  [USER_STORE.MUTATIONS.changeActiveSemesterType](state) {
    const current_semester = state.semesters[state.active_semester];
    if (current_semester.name.toString().includes("קיץ")) {
      current_semester.name = "0";
    } else {
      current_semester.name = "קיץ";
    }
    renameSemesters(state.semesters);
    state.summer_semesters = getSummerSemestersNumber(state.semesters);
  },
  [USER_STORE.MUTATIONS.changeSemesterType](state: UserState, index) {
    const semester = state.semesters[index];
    if (semester.name.toString().includes("קיץ")) {
      semester.name = "0";
    } else {
      semester.name = "קיץ";
    }
    renameSemesters(state.semesters);
    state.summer_semesters = getSummerSemestersNumber(state.semesters);
  },
  [USER_STORE.MUTATIONS.changeCategoryName](state: UserState, name_index) {
    if (name_index[1] < state.course_types.length) {
      state.course_types[name_index[1]].name = name_index[0];
    }
    calculateUserInfo(state);
  },
  [USER_STORE.MUTATIONS.deleteCourseType](state: UserState, index) {
    if (index < state.course_types.length) {
      resetRemovedCategory(state, index);
      state.course_types.splice(index, 1);
    }
    calculateUserInfo(state);
  },
  [USER_STORE.MUTATIONS.moveCourseToSemester](
    state: UserState,
    { semester_index, course_index }
  ) {
    const course_to_move =
      state.semesters[state.active_semester].courses[course_index];
    state.semesters[state.active_semester].courses.splice(course_index, 1);
    let index = 0;
    for (; index < state.semesters[semester_index].courses.length; index++) {
      if (state.semesters[semester_index].courses[index].isEmpty()) {
        break;
      }
    }
    state.semesters[semester_index].courses[index] = course_to_move;
  },
  [USER_STORE.MUTATIONS.reCalcCurrentSemester](state) {
    if (state.semesters.length > 0) {
      calculateUserInfo(state);
    }
  },
  [USER_STORE.MUTATIONS.checkForValidVersion](state) {
    if (state.course_types === []) {
      state.course_types = default_course_types_obj;
    }
  },
  [USER_STORE.MUTATIONS.updateSemester](state) {
    const user = auth.currentUser;
    if (user != null) {
      db.collection("users")
        .withConverter(stateConverter)
        .doc(user.uid)
        .update({
          semesters: state.semesters,
        })
        .then((r) => r);
    }
  },
  [USER_STORE.MUTATIONS.exportSemesters](state: UserState, with_grades) {
    const copy = JSON.stringify(state.semesters);
    const semesters: Semester[] = JSON.parse(copy);
    if (!with_grades) {
      for (const semester of semesters) {
        for (const course of semester.courses) {
          course.grade = 0;
        }
        semester.average = 0;
      }
    }
    const data = JSON.stringify(semesters, undefined, 2);
    saveJSON(data, "courses.json");
  },
  [USER_STORE.MUTATIONS.importCoursesFromJson](state: UserState, data) {
    state.semesters = JSON.parse(data);
  },
  [USER_STORE.MUTATIONS.fetchUserInfo](state: UserState, user) {
    state.summer_semesters = user.summer_semesters;
    state.active_semester = user.active_semester;
    state.degree_average = user.degree_average;
    state.degree_points = user.degree_points;
    state.degree_points_done = user.degree_points_done;
    state.degree_points_left = user.degree_points_left;
    state.degree_points_to_choose = user.degree_points_to_choose;
    state.english_exemption = user.english_exemption;
    state.semesters = user.semesters;
    state.course_types = user.course_types;
  },
  [USER_STORE.MUTATIONS.checkIfCourseExists](
    state: UserState,
    course_number_and_answer
  ) {
    course_number_and_answer["answer"] = courseExistInSemesters(
      state.semesters,
      course_number_and_answer.course_number
    );
  },
  [USER_STORE.MUTATIONS.checkPrerequisites](
    state: UserState,
    course_number_and_answer
  ) {
    course_number_and_answer["answer"] = courseExistInSemesters(
      state.semesters,
      course_number_and_answer.course_number,
      state.active_semester - 1
    );
  },
  [USER_STORE.MUTATIONS.checkLinear](
    state: UserState,
    course_number_and_answer
  ) {
    course_number_and_answer["answer"] = courseExistInSemesters(
      state.semesters,
      course_number_and_answer.course_number,
      state.active_semester
    );
  },

  [USER_STORE.MUTATIONS.updateSemesters](state: UserState, semesters) {
    state.semesters = semesters;
  },
};
