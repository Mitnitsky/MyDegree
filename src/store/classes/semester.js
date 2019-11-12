import {createNewCourse} from "@/store/classes/course";



export function createNewSemester(name, courses_initially)
{
    let semester = {};
    semester.name = name.toString();
    semester.average = 0;
    semester.points = 0;
    semester.courses = [];
    for (let i = 0; i < courses_initially; i++) {
        semester.courses.push(createNewCourse());
    }
    return semester;
}

export function addCourseToSemester(semester)
{
    semester.courses.push(createNewCourse());
}

export function addExistingCourse(semester, course)
{
    semester.courses.push(course);
}

export function removeCourse(semester, index)
{
    if (index < semester.courses.length && index >= 0) {
        semester.semester.splice(index, 1);
    }
}

export function calculateAverage(semester)
{
    let points = 0;
    let total_grade = 0;
    for (const course of semester.courses) {
        if (course.points !== '' && course.grade !== '') {
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

export function calculatePoints(semester)
{
    semester.points = 0;
    for (const course of semester.courses) {
        if (course.points !== '') {
            semester.points += parseFloat(course.points);
        }
    }
    semester.points.toFixed(1);
}