import { Course } from "@/store/classes/course";
import { Semester } from "@/store/classes/semester";
import { UserState } from "@/store/interfaces";
import { CourseType } from "@/store/classes/course_types";

export const stateConverter = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  toFirestore: (state: UserState): any => {
    return {
      summer_semesters: state.summer_semesters,
      active_semester: state.active_semester,
      degree_average: state.degree_average,
      degree_points: state.degree_points,
      degree_points_done: state.degree_points_done,
      degree_points_left: state.degree_points_left,
      degree_points_to_choose: state.degree_points_to_choose,
      english_exemption: state.english_exemption,
      semesters: state.semesters.map(semesterDataConverter.toObject),
      course_types: state.course_types.map(courseTypeDataConverter.toObject),
    };
  },
  // eslint-disable-next-line @typescript-eslint/explicit-module-boundary-types
  fromFirestore: (snapshot, options): UserState => {
    const data = snapshot.data(options);
    const course_types = data.course_types.map(
      courseTypeDataConverter.fromObject
    );
    return {
      summer_semesters: 0,
      active_semester: 0,
      degree_average: 0,
      degree_points: 0,
      degree_points_done: 0,
      degree_points_left: 0,
      degree_points_to_choose: 0,
      english_exemption: false,
      semesters: data.semesters.map(semesterDataConverter.fromObject),
      course_types: course_types,
    };
  },
};

const courseDataConverter = {
  toObject: (course: Course) => {
    return {
      existsInDB: course.existsInDB,
      name: course.name,
      number: course.number,
      points: course.points,
      grade: course.grade,
      type: course.type,
      binary: course.binary,
    };
  },
  fromObject: (object): Course => {
    const course = new Course();
    course.existsInDB = object.existsInDB;
    course.name = object.name;
    course.number = object.number;
    course.points = object.points;
    course.grade = object.grade;
    course.type = object.type;
    course.binary = object.binary;
    return course;
  },
};

const semesterDataConverter = {
  toObject: (semester: Semester) => {
    return {
      name: semester.name,
      average: semester.average,
      points: semester.points,
      courses: semester.courses.map(courseDataConverter.toObject),
    };
  },
  fromObject: (object) => {
    const semester = new Semester(object.name, 0);
    semester.name = object.name;
    semester.average = object.average;
    semester.points = object.points;
    semester.courses = object.courses.map(courseDataConverter.fromObject);
    return semester;
  },
};

const courseTypeDataConverter = {
  toObject: (type: CourseType) => {
    return {
      name: type.name,
      total_points: type.total_points,
      points_left: type.points_left,
      points_required: type.points_required,
      points_done: type.points_done,
      average: type.average,
    };
  },
  fromObject: (object) => {
    const type = new CourseType();
    type.name = object.name;
    type.total_points = object.total_points;
    type.points_left = object.points_left;
    type.points_required = object.points_required;
    type.points_done = object.points_done;
    type.average = object.average;
    return type;
  },
};
