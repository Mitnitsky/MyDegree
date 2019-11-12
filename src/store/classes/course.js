
export function createNewCourse() {
        let course = {};
        course.existsInDB = false;
        course.name = 'OS';
        course.number = '234123';
        course.points = 3;
        course.grade = 100;
        course.type = '1';
        return course;
}

export function courseIsEmpty(course) {
        return course.name !== '' && course.number !== '';
    }

export function clearCourse(course) {
        course.existsInDB = false;
        course.name = '';
        course.number = '';
        course.points = '';
        course.grade = '';
        course.type = '0';
    }

