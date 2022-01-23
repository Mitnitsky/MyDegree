import sqlite3
from multiprocessing import Pool
import sys
import time
import json

from db_helpers import *
from parse_students_site import *
from course import Course


def prepare_package(semester, faculty):
    """Preparing package with given semester and faculty in order to get courses from UG

    Args:
        semester (INT): semester number (e.g. "201802")
        faculty (INT): faculty  number parsed from UG html
    """
    post_package = {
        "CNM": "",
        "CNO": "",
        "PNT": "",
        "LLN": "",
        "LFN": "",
        "RECALL": "Y",
        "D1": "on",
        "D2": "on",
        "D3": "on",
        "D4": "on",
        "D5": "on",
        "D6": "on",
        "FTM": "",
        "TTM": "",
        "SIL": "",
        "OPTCAT": "on",
        "OPTSEM": "on",
        "doSearch": "Y",
        "Search": "חפש",
        "FAC": faculty,
        "SEM": semester,
    }
    return post_package


def can_be_added_as_prerequisite(course, inner_course, prerequisite_course_number):
    return inner_course['number'] == prerequisite_course_number and course['full_name'] not in inner_course[
        "followed_by"]


def find_all_courses_followed_by(courses):
    for course in courses:
        course['full_name'] = course['number'] + ': ' + course['name']
        for prerequisite_courses in course['prerequisites']:
            for prerequisite_course_fullname in prerequisite_courses:
                prerequisite_course_number = prerequisite_course_fullname.split(":")[0]
                for inner_course in courses:
                    if can_be_added_as_prerequisite(course, inner_course, prerequisite_course_number):
                        inner_course["followed_by"].append(course['full_name'])
        course.pop('english', None)


# Function which humanities dependencies
def cut_dependencies(dependencies):
    result = list()
    dependencies = list(map(str.strip, dependencies.split("|")))
    braces_remove = str.maketrans({"(": None, ")": None})
    for dependence in dependencies:
        temp = list()
        temp.extend(
            map(
                lambda x: x.translate(braces_remove),
                map(str.strip, dependence.split("&")),
            )
        )
        result.append(temp)
    return result


# Function which gets a course info from ug website
# Creates a course class instance and writes all the data into it
def get_course_info(course_number):
    print(course_number)
    temp_course = Course()
    types = get_info_from_students(course_number)
    temp_course.add_prerequisites(types[0])
    temp_course.add_linked(types[1])
    temp_course.add_identical(types[2])
    temp_course.add_overlapping(types[3])
    temp_course.add_inclusive(types[4])
    temp_course.add_including(types[5])
    temp_course.set_points(types[6])
    temp_course.set_name(types[7])
    temp_course.set_number(types[8])
    return temp_course


# Function which updates the Courses database
def update_db_from_ug(db):
    init_db(db)
    course_numbers = sorted(get_all_courses_numbers())
    pool = Pool(processes=16)
    results = [pool.apply_async(get_course_info, args=(course_number,)) for course_number in course_numbers]
    output = [p.get() for p in results]
    for course in output:
        db_add_course(db, course)


# Function which creates possible packages to prompt ug for sport course with the given semester
def sport_packages(semester):
    package = prepare_package(semester, "")
    package2 = prepare_package(semester, "")
    package2["CNM"] = "חינוך גופני"
    package["CNM"] = "ספורט"
    return [package, package2]


def convert_db_to_json(db):
    types = ['linked', 'identical', 'overlapping', 'inclusive', 'including', "followed_by"]
    courses = extract_all_courses_with_types_from_db(db, types)
    find_all_courses_followed_by(courses)
    with open('db/courses.json', 'w') as courses_file:
        courses_file.write(json.dumps({"courses": courses}, indent=4, ensure_ascii=False))


def main():
    sys.setrecursionlimit(250000)
    start_time = time.time()
    with closing(sqlite3.connect("./db/courses.db")) as db:
        update_db_from_ug(db)
        convert_db_to_json(db)
    print("--- %.2f seconds ---" % (time.time() - start_time))


if __name__ == "__main__":
    main()
