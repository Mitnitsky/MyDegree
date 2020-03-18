export function createNewCourse() {
  let course = {};
  course.existsInDB = false;
  course.name = "";
  course.number = "";
  course.points = 0;
  course.grade = 0;
  course.type = 0;
  return course;
}

export function createCourseFromDBEntry(course) {
  let course_to_add = {};
  course_to_add.existsInDB = true;
  course_to_add.name = course.name;
  course_to_add.number = course.number;
  course_to_add.points = course.points;
  if (course.grade !== "undefined") {
    course_to_add.grade = isNaN(parseInt(course.grade))
      ? 0
      : parseInt(course.grade);
  } else {
    course_to_add.grade = 0;
  }
  course_to_add.type = 0;

  return course_to_add;
}

export function courseIsEmpty(course) {
  return (
    course.name === "" &&
    (course.number === "" || parseInt(course.number) === 0)
  );
}

export function clearCourse(course) {
  course.existsInDB = false;
  course.name = "";
  course.number = "";
  course.points = 0;
  course.grade = 0;
  course.type = 0;
}
