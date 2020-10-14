
export function createNewCourse() {
  let course = {};
  course.existsInDB = false;
  course.name = "";
  course.number = "";
  course.points = 0;
  course.grade = 0;
  course.type = 0;
  course.binary = false;
  return course;
}

export function createCourseFromDBEntry(course) {
  const exemption_index = 1;
  let course_to_add = {};
  course_to_add.existsInDB = true;
  course_to_add.name = course.name;
  course_to_add.number = course.number;
  course_to_add.points = course.points;
  course_to_add.type = 0;
  course_to_add.binary = false;
  if (course.grade !== undefined) {
    course_to_add.grade = isNaN(parseInt(course.grade))
      ? 0
      : parseInt(course.grade);
    if(course.grade.includes('פטור')){
      course_to_add.type = exemption_index;
    }
  } else {
    course_to_add.grade = 0;
  }
  return course_to_add;
}

export function courseIsEmpty(course) {
  return (
    course.name === "" &&
    (course.number === "" || parseInt(course.number) === 0)
  );
}
