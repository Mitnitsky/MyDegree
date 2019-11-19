import {courseIsEmpty, createCourseFromDBEntry, createNewCourse} from "@/store/classes/course";
import {course_types} from "./course_types";

export function createNewSemester(name, courses_initially) {
    let semester = {};
    semester.name = name.toString();
    semester.average = 0;
    semester.points = 0;
    semester.points_done = 0;
    semester.must_points = 0;
    semester.a_list_points = 0;
    semester.b_list_points = 0;
    semester.humanistic_points = 0;
    semester.free_points = 0;
    semester.projects_points = 0;
    semester.sport = 0;
    semester.exemption_points = 0;
    semester.courses = [];
    for (let i = 0; i < courses_initially; i++) {
        semester.courses.push(createNewCourse());
    }
    return semester;
}

export function addCourseToSemester(semester) {
    semester.courses.push(createNewCourse());
}

export function addExistingCourse(semester, course) {
    for (let i = 0; i < semester.courses.length; i++) {
        window.console.log(i);
        if (courseIsEmpty(semester.courses[i])) {
            semester.courses[i].name = course.name;
            semester.courses[i].points = course.points;
            semester.courses[i].number = course.number;
            calculateAverage(semester);
            calculatePoints(semester);
            return;
        }
    }
    //No empty place found.
    semester.courses.push(createCourseFromDBEntry(course));
    calculateAverage(semester);
    calculatePoints(semester);
}

export function removeCourse(semester, index) {
    if (index < semester.courses.length && index >= 0) {
        semester.courses.splice(index, 1);
    }
    calculateAverage(semester);
    calculatePoints(semester);
}

export function calculateAverage(semester) {
    if (semester !== 'undefined')  {
        let points = 0;
        let total_grade = 0;
        for (const course of semester.courses) {
            if (course.points !== ''
                && course.grade !== ''
                && parseFloat(course.grade) !== 0
                && parseFloat(course.points) !== 0
                && course.type !== course_types.EXEMPTION) {
                points += parseFloat(course.points);
                total_grade += parseFloat(course.grade) * parseFloat(course.points);
            }
        }
        if (points !== 0) {
            if (parseInt((total_grade / points)) == (total_grade / points)) {
                semester.average = parseInt(total_grade / points);
            } else {
                semester.average = (total_grade / points).toFixed(2);
            }
        } else {
            semester.average = 0;
        }
    }
}

export function calculatePoints(semester) {
    if(semester !== 'undefined') {
        semester.points = 0;
        semester.points_done = 0;
        semester.must_points = 0;
        semester.a_list_points = 0;
        semester.b_list_points = 0;
        semester.humanistic_points = 0;
        semester.free_points = 0;
        semester.projects_points = 0;
        semester.sport = 0;
        semester.exemption_points = 0;
        for (const course of semester.courses) {
            if (course.points !== '') {
                switch (course.type) {
                    case course_types.MUST:
                        semester.must_points += parseFloat(course.points);
                        break;
                    case course_types.LIST_A:
                        semester.a_list_points += parseFloat(course.points);
                        break;
                    case course_types.LIST_B:
                        semester.b_list_points += parseFloat(course.points);
                        break;
                    case course_types.HUMANISTIC:
                        semester.humanistic_points += parseFloat(course.points);
                        break;
                    case course_types.FREE_CHOICE:
                        semester.free_points += parseFloat(course.points);
                        break;
                    case course_types.PROJECTS:
                        semester.projects_points += parseFloat(course.points);
                        break;
                    case course_types.SPORT:
                        semester.sport += parseFloat(course.points);
                        break;
                    case course_types.EXEMPTION:
                        semester.exemption_points += parseFloat(course.points);
                        semester.points_done += parseFloat(course.points);
                        semester.points += parseFloat(course.points);
                        continue;
                }
                semester.points += parseFloat(course.points);
                if (course.grade !== '' && parseFloat(course.grade) !== 0) {
                    semester.points_done += parseFloat(course.points);
                }
            }
        }
        semester.points.toFixed(1);
    }
}

export function hasCourse(semester, course_number) {
    for (const course of semester.courses) {
        if (course.number.toString() === course_number.toString()) {
            return true;
        }
    }
    return false;
}

export function courseExistInSemesters(semesters, course_number, stop_index = null) {
    if (stop_index === null) {
        stop_index = semesters.length - 1;
    }
    for (let index = 0; index <= stop_index; index++) {
        if (hasCourse(semesters[index], course_number)) {
            return semesters[index].name;
        }
    }
    return false;
}