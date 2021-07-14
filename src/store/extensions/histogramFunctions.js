import $ from "jquery";

export function convertTechnionSemesterToText(semester) {
  const semesters = ["חורף", "אביב", "קיץ"];
  const year = parseInt(semester.slice(0, 4));
  const semester_number = parseInt(semester.slice(5, 6)) - 1;
  if (semester_number === 0) {
    return (
      semesters[semester_number] +
      " " +
      year.toString() +
      "-" +
      (year + 1).toString()
    );
  } else {
    return semesters[semester_number] + " " + (year + 1).toString();
  }
}

export function convertExamNameToHebrew(exam_name) {
  let exam_name_lower = exam_name.toLowerCase();
  if (exam_name_lower === "Final_A".toLowerCase()) {
    return "סופי מועד א'";
  } else if (exam_name_lower === "Final_B".toLowerCase()) {
    return "סופי מועד ב'";
  } else if (exam_name_lower === "Finals".toLowerCase()) {
    return "סופי";
  } else if (exam_name_lower === "Exam_A".toLowerCase()) {
    return "מבחן מועד א'";
  } else if (exam_name_lower === "Exam_B".toLowerCase()) {
    return "מבחן מועד ב'";
  } else {
    return exam_name;
  }
}

export function convertJsonToProperSelectBoxFormat(json_obj) {
  let course_info = [];
  for (let semester of Object.keys(json_obj)) {
    let semester_result = {};
    semester_result.label = convertTechnionSemesterToText(semester);
    semester_result.semester_number = semester;
    semester_result.options = [];
    let staff = "";
    for (let entry of Object.keys(json_obj[semester])) {
      if (entry.startsWith("Staff")) {
        staff =
          json_obj[semester][entry][0].title +
          ": " +
          json_obj[semester][entry][0].name;
        continue;
      }
      let entry_result = {};
      entry_result.value = [json_obj[semester][entry]];
      entry_result.value[0].semester_name = semester_result.label;
      entry_result.value[0].semester_number = semester;
      entry_result.value[0].entry_name = entry;
      if (staff.length > 0) {
        entry_result.value[0].staff = staff;
      }
      entry_result.text = convertExamNameToHebrew(entry);
      semester_result.options.push(entry_result);
    }
    course_info.push(semester_result);
  }

  return course_info.sort(function (a, b) {
    return b.index - a.index;
  });
}

export function getHistogramForCourseNumber(course_number) {
  let json = null;
  $.ajax({
    dataType: "json",
    url: `https://michael-maltsev.github.io/technion-histograms/${course_number}/index.json`,
    async: false,
    success: function (doc) {
      json = doc;
    },
  });
  return convertJsonToProperSelectBoxFormat(json);
}
