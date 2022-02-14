import { Semester } from "./semester";
import { CourseType, default_course_types_obj } from "./course_types";

export class UserData {
  summer_semesters = 0;
  active_semester = 0;
  degree_average = 0;
  degree_points = 0;
  degree_points_done = 0;
  degree_points_left = 0;
  degree_points_to_choose = 0;
  english_exemption = false;
  semesters: Semester[] = [];
  course_types: CourseType[] = default_course_types_obj;
}
