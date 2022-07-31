import { UserState } from '@/store/interfaces'
import { default_course_types_obj } from '@/store/classes/course_types'

export const state: UserState = {
  summer_semesters: 0,
  active_semester: 0,
  degree_average: 0,
  degree_points: 0,
  degree_points_done: 0,
  degree_points_left: 0,
  degree_points_to_choose: 0,
  english_exemption: false,
  semesters: [],
  course_types: default_course_types_obj,
}
