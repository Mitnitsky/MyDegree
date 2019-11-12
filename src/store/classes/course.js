
export function createNewCourse() {
        let course = {};
        course.existsInDB = false;
        course.name = '';
        course.number = '';
        course.points = 0;
        course.grade = 0;
        course.type = '0';
        return course;
}

export function courseIsEmpty(course) {
        return course.name == '' && (course.number === ''  || parseInt(course.number) === 0);
    }

export function clearCourse(course) {
        course.existsInDB = false;
        course.name = '';
        course.number = '';
        course.points = '';
        course.grade = '';
        course.type = '0';
    }

