export class Course {
  existsInDB = false;
  name = "";
  number = "";
  points = 0.0;
  grade = 0;
  type = 0;
  binary = false;

  clear(): void {
    this.existsInDB = false;
    this.name = "";
    this.number = "";
    this.points = 0;
    this.grade = 0;
    this.type = 0;
    this.binary = false;
  }
  isEmpty(): boolean {
    return (
      this.name === "" && (this.number === "" || parseInt(this.number) === 0)
    );
  }
}

export function createCourseFromDBEntry(course: Course): Course {
  const course_to_add = new Course();
  course_to_add.existsInDB = true;
  course_to_add.name = course.name;
  course_to_add.number = course.number;
  course_to_add.points = course.points;
  course_to_add.type = course.type;
  course_to_add.binary = false;
  course_to_add.grade = course.grade;
  return course_to_add;
}

export function compareByNumericField(
  a: Course,
  b: Course,
  fieldName: string
): number {
  if (fieldName == "number") {
    if (a.number > b.number) {
      return 1;
    } else if (a.number < b.number) {
      return -1;
    } else {
      return 0;
    }
  } else if (fieldName == "grade") {
    if (a.grade > b.grade) {
      return 1;
    } else if (a.grade < b.grade) {
      return -1;
    } else {
      return 0;
    }
  } else if (fieldName == "points") {
    if (a.points > b.points) {
      return 1;
    } else if (a.points < b.points) {
      return -1;
    } else {
      return 0;
    }
  }
  return 0;
}

export function is_courses_array_sorted(
  arr: Course[],
  fieldName: string
): boolean {
  for (let i = 0; i < arr.length - 1; i++) {
    if (fieldName === "name") {
      if (CharCompare(arr[i].name, arr[i + 1].name, 0) === 1) {
        return false;
      }
    } else {
      if (arr[i + 1].name.toString() === "") {
        continue;
      }
      if (compareByNumericField(arr[i], arr[i + 1], fieldName) === 1) {
        return false;
      }
    }
  }
  return true;
}

export function CharCompare(a: string, b: string, index: number): number {
  const alphabets = [
    " ",
    "-",
    ",",
    "'",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "a",
    "b",
    "c",
    "d",
    "e",
    "f",
    "g",
    "h",
    "i",
    "k",
    "l",
    "m",
    "n",
    "o",
    "p",
    "q",
    "r",
    "s",
    "t",
    "v",
    "u",
    "w",
    "x",
    "y",
    "z",
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
  if (index === a.length || index === b.length) return 0;
  const aChar = alphabets.indexOf(a.toUpperCase().charAt(index));
  const bChar = alphabets.indexOf(b.toUpperCase().charAt(index));
  if (aChar !== bChar) {
    return aChar - bChar > 0 ? 1 : -1;
  } else {
    return CharCompare(a, b, index + 1);
  }
}
