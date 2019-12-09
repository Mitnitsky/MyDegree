import {courseIsEmpty, createCourseFromDBEntry, createNewCourse} from "@/store/classes/course";
import {course_types} from "./course_types";
import {MathRound10} from "../aux/rounder";

function initializeSemesterPoints(semester) {
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
}

export function createNewSemester(name, courses_initially) {
    let semester = {};
    semester.name = name.toString();
    semester.average = 0;
    initializeSemesterPoints(semester);
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
        if (courseIsEmpty(semester.courses[i])) {
            semester.courses[i].name = course.name;
            semester.courses[i].points = course.points;
            semester.courses[i].number = course.number;
            if (course.grade !== 'undefined') {
                semester.courses[i].grade = parseInt(course.grade);
                if (isNaN(semester.courses[i].grade)) {
                    semester.courses[i].grade = 0;
                }
            } else {
                semester.courses[i].grade = 0;
            }
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
    if (semester !== 'undefined') {
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
                semester.average = MathRound10(total_grade / points, -1);
            }
        } else {
            semester.average = 0;
        }
    }
}

export function calculatePoints(semester) {
    if (semester !== 'undefined') {
        initializeSemesterPoints(semester)
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

function compareByNumericField(a, b, fieldName) {
    if (isNaN(parseInt(a[fieldName]))) {
        return 1;
    } else if (isNaN(parseInt(b[fieldName]))) {
        return -1;
    }
    if (parseInt(a[fieldName]) > parseInt(b[fieldName])) {
        return 1;
    } else if (parseInt(a[fieldName]) < parseInt(b[fieldName])) {
        return -1
    }
    return 0;
}

function is_array_sorted(arr, fieldName) {
    for (let i = 0; i < arr.length - 1; i++) {
        if (fieldName === 'name') {
            if (CharCompare(arr[i].name, arr[i + 1].name, 0) === 1) {
                return false;
            }
        } else {
            if (compareByNumericField(arr[i], arr[i + 1], fieldName) === 1) {
                return false;
            }
        }
    }
    return true;
}


/**
 * @return {number}
 */
function CharCompare(a, b, index) {
    let alphabets = [" ", "-", ",", "'", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "v", "u", "w", "x", "y", "z",
        "א",
        "ב",
        "ג",
        "ד",
        "ה",
        "ו",
        "ז",
        "ח",
        "ט",
        "י",
        "כ",
        "ל",
        "מ",
        "ם",
        "נ",
        "ן",
        "ס",
        "ע",
        "פ",
        "ף",
        "צ",
        "ץ",
        "ק",
        "ר",
        "ש",
        "ת",
    ];
    if (index === a.length || index === b.length)
        return 0;
    let aChar = alphabets.indexOf(a.toUpperCase().charAt(index));
    let bChar = alphabets.indexOf(b.toUpperCase().charAt(index));
    if (aChar !== bChar) {
        return (aChar - bChar > 0) ? 1 : -1;
    } else {
        return CharCompare(a, b, index + 1)
    }
}


export function sortCoursesByField(semester, fieldName) {
    if (semester) {
        if (semester.courses) {
            if (fieldName === 'name') {
                if (is_array_sorted(semester.courses, fieldName)) {
                    semester.courses.sort((a, b) => {
                        return (CharCompare(a.name, b.name, 0) * -1)
                    })
                } else {
                    semester.courses.sort((a, b) => {
                        return (CharCompare(a.name, b.name, 0))
                    })
                }
            } else {
                if (is_array_sorted(semester.courses, fieldName)) {
                    semester.courses.sort((a, b) => {
                        return (compareByNumericField(a, b, fieldName) * -1)
                    })
                } else {
                    semester.courses.sort((a, b) => {
                        return (compareByNumericField(a, b, fieldName))
                    })
                }
            }
            semester.courses.sort((a, b) => {
                if (a.number.toString() === '' || a.name.toString() === '') {
                    return 1
                } else if (b.number.toString() === '' || a.name.toString() === '') {
                    return -1
                } else return 0
            })
        }
    }
}

