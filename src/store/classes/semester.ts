import {
  Course,
  CharCompare,
  compareByNumericField,
  is_courses_array_sorted,
  createCourseFromDBEntry,
} from "./course";
import { MathRound10 } from "../extensions/rounder";

const exemption_index = 1;

export class Semester {
  name = "";
  average = 0.0;
  points = 0.0;
  courses: Course[] = [];

  toString(): string {
    return (
      this.name +
      ", " +
      this.average +
      ", " +
      this.points +
      "\n" +
      this.courses.join("\n")
    );
  }

  constructor(name: string, empty_courses: number) {
    this.name = name;
    for (let i = 0; i < empty_courses; i++) {
      this.courses.push(new Course());
    }
  }

  addCourse(): void {
    this.courses.push(new Course());
  }

  addExistingCourseReturnIndex(course: Course): number {
    let addedIndex = -1;
    for (let i = 0; i < this.courses.length; i++) {
      if (this.courses[i].isEmpty()) {
        this.courses[i] = course;
        addedIndex = i;
        break;
      }
    }
    if (addedIndex == -1) {
      this.courses.push(createCourseFromDBEntry(course));
      addedIndex = this.courses.length - 1;
    }
    this.calculateAverage();
    this.calculatePoints();
    return addedIndex;
  }

  sortCoursesByField(fieldName: "number" | "grade" | "points" | "name"): void {
    if (this.courses.length == 0) {
      return;
    }
    if (fieldName === "name") {
      if (is_courses_array_sorted(this.courses, fieldName)) {
        this.courses.sort((a, b) => {
          return CharCompare(a.name, b.name, 0) * -1;
        });
      } else {
        this.courses.sort((a, b) => {
          return CharCompare(a.name, b.name, 0);
        });
      }
    } else {
      if (is_courses_array_sorted(this.courses, fieldName)) {
        this.courses.sort((a, b) => {
          return compareByNumericField(a, b, fieldName) * -1;
        });
      } else {
        this.courses.sort((a, b) => {
          return compareByNumericField(a, b, fieldName);
        });
      }
    }
    this.courses.sort((a, b) => {
      if (a.number.toString() === "" && a.name.toString() === "") {
        return 1;
      } else if (b.number.toString() === "" && b.name.toString() === "") {
        return -1;
      } else return 0;
    });
  }

  calculateAverage(): void {
    let points = 0;
    let binary_points = 0;
    let total_grade = 0;
    for (const course of this.courses) {
      if (course.binary) {
        binary_points += course.points;
        points += course.points;
        continue;
      }
      if (course.type === exemption_index || course.grade === 0) {
        continue;
      }
      total_grade += course.grade * course.points;
      points += course.points;
    }
    const points_graded = points - binary_points;
    if (points_graded !== 0) {
      if (
        parseInt((total_grade / points_graded).toString()) ===
        total_grade / points_graded
      ) {
        this.average = parseInt((total_grade / points_graded).toString());
      } else {
        this.average = MathRound10(total_grade / points_graded, -1);
      }
    } else {
      this.average = 0;
    }
  }

  removeCourse(index: number): void {
    if (index >= this.courses.length || index < 0) {
      return;
    }
    this.courses.splice(index, 1);
    this.calculateAverage();
    this.calculatePoints();
  }

  calculatePoints(): void {
    this.points = 0;
    for (const course of this.courses) {
      this.points += course.points;
    }
    this.points.toFixed(1);
  }

  hasCourse(courseNumber: string): boolean {
    for (const course of this.courses) {
      if (course.number.toString() === courseNumber) {
        return true;
      }
    }
    return false;
  }
}

export function courseExistInSemesters(
  semesters: Semester[],
  course_number: string,
  stop_index: number | null = null
): number | string {
  if (stop_index === null) {
    stop_index = semesters.length - 1;
  }
  let summer_semesters = 0;
  let found_index: number | string = -1;
  for (let index = 0; index <= stop_index; index++) {
    if (semesters[index].name.toString().includes("קיץ")) {
      summer_semesters += 1;
    }
    if (semesters[index].hasCourse(course_number)) {
      found_index = index + 1 - summer_semesters;
      if (semesters[index].name.toString().includes("קיץ")) {
        found_index = "קיץ";
      }
    }
  }
  return found_index;
}

export function renameSemesters(semesters: Semester[]): void {
  if (!semesters || semesters.length < 0) {
    return;
  }
  let summer_semesters = 0;
  for (let i = 0; i < semesters.length; i++) {
    if (semesters[i].name.toString().includes("קיץ")) {
      summer_semesters++;
    } else {
      semesters[i].name = (1 + i - summer_semesters).toString();
    }
  }
}

export function getSummerSemestersNumber(semesters: Semester[]): number {
  if (!semesters || semesters.length < 0) {
    return 0;
  } else {
    let summer_semesters = 0;
    for (let i = 0; i < semesters.length; i++) {
      if (semesters[i].name.toString().includes("קיץ")) {
        summer_semesters++;
      }
    }
    return summer_semesters;
  }
}
