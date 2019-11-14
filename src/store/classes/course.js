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

export function courseIsEmpty(course) {
    return course.name == '' && (course.number === '' || parseInt(course.number) === 0);
}

export function clearCourse(course) {
    course.existsInDB = false;
    course.name = '';
    course.number = '';
    course.points = 0;
    course.grade = 0;
    course.type = course_types.MUST;
}

