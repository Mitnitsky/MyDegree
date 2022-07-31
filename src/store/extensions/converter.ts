import { Semester } from '@/store/classes/semester'
import { Course } from '@/store/classes/course'
import { JsonCourse } from '@/store/classes/json_course_db'

function includesOneOf(str: string, ...args: string[]) {
  return args.some((v) => str.includes(v))
}

function parseCourseLine(course: Course, parts: string[]) {
  course.number = parts[0].trim()
  course.name = parts[1].trim()
  course.points = parseInt(parts[2].trim())
  course.grade = parseInt(
    parts[3].split('-').join('').split('*').join('').replace('לא השלים', '').trim()
  )
  if (isNaN(course.grade)) {
    course.grade = 0
  }
}

export function parseStudentsSiteGrades(grades_copy_str: string): {
  semesters: Semester[]
  english_exemption: boolean
  summer_semesters_indexes: number[]
} {
  const grades_copy = grades_copy_str.split('\n')
  const raw_semesters: string[][] = [[]]
  let semesters_index = 0
  let found_first_semester = false
  let english_exemption = false
  let exempted_courses_part_found = false
  const semesters: Semester[] = []
  const summer_semesters_indexes: number[] = []
  const exempted_courses: string[] = []
  for (const line of grades_copy) {
    if (!found_first_semester) {
      if (includesOneOf(line, 'אנגלית', 'פטור')) {
        english_exemption = true
      }
      if (exempted_courses_part_found) {
        if (
          !includesOneOf(
            line,
            'ציון',
            'ממוצע',
            'הצלחות',
            'לא השלים',
            'סה"כ',
            'ממוצע סמסטר',
            'הצלחות סמסטר',
            'נקודות רישום:',
            'סמסטר'
          ) &&
          line.length > 0
        ) {
          exempted_courses.push(line)
        }
      }
      if (line.includes('זיכויים')) {
        exempted_courses_part_found = true
      }
      if (includesOneOf(line, 'קיץ', 'חורף', 'אביב')) {
        if (line.includes('קיץ')) {
          summer_semesters_indexes.push(0)
        }
        found_first_semester = true
      }
      continue
    }
    if (includesOneOf(line, 'קיץ', 'חורף', 'אביב')) {
      semesters_index += 1
      if (line.includes('קיץ')) {
        summer_semesters_indexes.push(semesters_index)
      }
      raw_semesters.push([])
      continue
    }
    if (
      !includesOneOf(
        line,
        'ציון',
        'ממוצע',
        'הצלחות',
        'לא השלים',
        'סה"כ',
        'ממוצע סמסטר',
        'הצלחות סמסטר',
        'נקודות רישום:'
      ) &&
      line.length > 0
    ) {
      raw_semesters[semesters_index].push(line)
    }
  }
  semesters_index = 1
  const exemption_type_index = 3
  const exempted_course_name_index = 1
  for (const rawSemester of raw_semesters) {
    const courses: Course[] = []
    if (semesters_index === 1 && exempted_courses.length > 0) {
      for (const exempted_course of exempted_courses) {
        const exempted_course_parts = exempted_course.split('\t')
        if (exempted_course_parts.length !== 4) {
          continue
        }
        if (
          exempted_course_parts[exemption_type_index].includes('פטור עם ניקוד') &&
          !exempted_course_parts[exempted_course_name_index].includes('אנגלית')
        ) {
          const course = new Course()
          parseCourseLine(course, exempted_course_parts)
          courses.push(course)
        }
      }
    }
    for (const raw_line of rawSemester) {
      let course: Course | null = new Course()
      if (raw_line.trim().length > 1) {
        const parts = raw_line.split('\t')
        if (parts.length !== 4) {
          continue
        }
        parseCourseLine(course, parts)
        for (const already_added_course of courses) {
          if (already_added_course.name === course?.name) {
            already_added_course.grade = course?.grade
            course = null
          }
        }
        if (course !== null) {
          courses.push(course)
        }
      }
    }
    const semester = new Semester(semesters_index.toString(), 0)
    semester.courses = courses
    semester.calculatePoints()
    semester.calculateAverage()
    semester.sortCoursesByField('number')
    semesters.push(semester)
    semesters_index += 1
  }
  return { semesters, english_exemption, summer_semesters_indexes }
}

function createCourseFromParts(course: Course, parts: string[]) {
  course.grade = parseFloat(
    parts[0].split('-').join('').split('*').join('').replace('לא השלים', '').trim()
  )
  if (isNaN(course.grade)) {
    course.grade = 0
  }
  course.points = parseFloat(parts[1].trim())
  const course_full_name = parts[2].split(' ')
  course.name = course_full_name.slice(0, -1).join(' ').trim()
  course.number = course_full_name[course_full_name.length - 1].trim()
}

export function parseGraduateInformation(grades_copy_str: string): {
  semesters: Semester[]
  exemption: boolean
  summer_semesters_indexes: number[]
} {
  const grades_copy: string[] = grades_copy_str.split('\n')
  const lines: string[][] = [[]]
  let index = 0
  let found_first_sem = false
  let english_exemption = false
  const semesters: Semester[] = []
  const summer_semester_indexes: number[] = []
  const exempted_courses: Course[] = []
  for (const line of grades_copy) {
    if (!found_first_sem) {
      if (line.includes('אנגלית') && line.includes('פטור')) {
        english_exemption = true
      }
      if (line.includes('פטור עם ניקוד') && !line.includes('אנגלית')) {
        const parts = line.split('\t')
        const course: Course = new Course()
        createCourseFromParts(course, parts)

        exempted_courses.push(course)
      }
      if (line.includes('קיץ') || line.includes('חורף') || line.includes('אביב')) {
        if (line.includes('קיץ')) {
          summer_semester_indexes.push(0)
        }
        found_first_sem = true
      }
    } else {
      if (line.includes('קיץ') || line.includes('חורף') || line.includes('אביב')) {
        index += 1
        if (line.includes('קיץ')) {
          summer_semester_indexes.push(index)
        }
        lines.push([])
        continue
      }
      if (
        !line.includes('ציון') &&
        !line.includes('ממוצע') &&
        !line.includes('הצלחות') &&
        !line.includes('לא השלים') &&
        !line.includes('סה"כ')
      ) {
        lines[index].push(line)
      }
    }
  }
  index = 1
  for (const semester of lines) {
    const courses: Course[] = []
    for (const line of semester) {
      let course: Course | null = new Course()
      if (line.length > 1 && line.trim().length > 1) {
        const parts = line.split('\t')
        createCourseFromParts(course, parts)

        for (const already_added_course of courses) {
          if (already_added_course.name === course?.name) {
            already_added_course.grade = course?.grade
            course = null
          }
        }
        if (course !== null) {
          courses.push(course)
        }
      }
    }
    semesters.push(new Semester(index.toString(), 0))
    if (index === 1) {
      for (const course of exempted_courses.concat(courses)) {
        semesters[index - 1].addExistingCourseReturnIndex(course)
      }
    } else {
      for (const course of courses) {
        semesters[index - 1].addExistingCourseReturnIndex(course)
      }
    }
    index += 1
  }
  return {
    semesters: semesters,
    exemption: english_exemption,
    summer_semesters_indexes: summer_semester_indexes,
  }
}

export function findCourse(course_number: string, json_courses: JsonCourse[]): JsonCourse[] {
  if (course_number.length < 3) {
    return []
  }
  if (json_courses['courses'] !== undefined) {
    return json_courses['courses'].filter((e: JsonCourse) => e.number.includes(course_number))
  } else {
    return json_courses.filter((e: JsonCourse) => e.number.includes(course_number))
  }
}

export function parseCheeseFork(courses_str: string): JsonCourse[] {
  const courses = courses_str.split('\n')
  const courses_from_db: JsonCourse[] = []
  let json_courses
  if (localStorage.getItem('courses')) {
    if (typeof localStorage.getItem('courses') === 'object') {
      json_courses = localStorage.getItem('courses')
    } else {
      const local_courses = localStorage.getItem('courses')
      if (local_courses !== null) {
        json_courses = JSON.parse(local_courses)
      } else {
        json_courses = {}
      }
    }
    if (!json_courses.version || json_courses.version < 7.0) {
      json_courses = require('../../data/courses.json')
      localStorage.setItem('courses', JSON.stringify(json_courses))
    }
  } else {
    json_courses = require('../../data/courses.json')
    localStorage.setItem('courses', JSON.stringify(json_courses))
  }
  const j_courses = json_courses.courses
  for (const course of courses) {
    const split = course.trim().split('-')
    if (split.length >= 2) {
      const course_number = split[0].trim()
      if (!isNaN(parseInt(course_number))) {
        const result = findCourse(course_number, j_courses)
        if (result.length > 0) {
          courses_from_db.push(result[0])
        }
      }
    }
  }
  return courses_from_db
}
