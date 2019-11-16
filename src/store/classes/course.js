import {course_types} from "./course_types";

export function createNewCourse() {
    let course = {};
    course.existsInDB = false;
    course.name = '';
    course.number = '';
    course.points = 0;
    course.grade = 0;
    course.type = course_types.MUST;
    return course;
}

export function createCourseFromDBEntry(course) {
    let course_to_add = {};
    course_to_add.existsInDB = true;
    course_to_add.name = course.name;
    course_to_add.number = course.number;
    course_to_add.points = course.points;
    course_to_add.grade = 0;
    course_to_add.type = course_types.MUST;
    return course_to_add;
}

export function courseIsEmpty(course) {
    return course.name === '' && (course.number === '' || parseInt(course.number) === 0);
}

export function clearCourse(course) {
    course.existsInDB = false;
    course.name = '';
    course.number = '';
    course.points = 0;
    course.grade = 0;
    course.type = course_types.MUST;
}

