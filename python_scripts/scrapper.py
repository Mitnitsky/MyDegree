import pickle
import sqlite3
import json
import bs4
from itertools import product
from datetime import date

import requests
from bs4 import BeautifulSoup
from parse_graduate_info import get_info_from_graduate
from course import Course


def preparePackage(SEM, FAC):
    """Preparing package with given semester and faculty in order to get courses from UG

    Args:
        SEM (INT): semester number (eg. "201802")
        FAC (INT): faculty  number parsed from UG html
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
        "FAC": FAC,
        "SEM": SEM,
    }
    return post_package


# Function which scraps ug website using the given package
# in order to get all the course numbers from the page
def getCourses(url, post_package):
    with requests.Session() as session:
        get = session.post(url, data=post_package)
        soup = BeautifulSoup(get.content, features="html5lib")
        selects = soup.find_all(lambda a: a.has_attr("href"))
        return uniqueAndSortInput(selects, "content")


# Function which gets all the possible semesters/faculties
# from ug website
def getData(url, tag, attrs, types):
    with requests.Session() as session:
        try:
            get = session.post(url)
        except (
                ConnectionAbortedError,
                ConnectionError,
                ConnectionRefusedError,
                ConnectionResetError,
        ):
            return []
        soup = BeautifulSoup(get.content, features="html5lib")
        selects = soup.find_all(tag, attrs)
        if "valign" in attrs.keys():
            data = []
            table = soup.find("table", attrs={"id": "points"})
            table_body = table.find("tbody")

            rows = table_body.find_all("tr")
            for row in rows:
                cols = row.find_all("td")
                cols = [ele.text.strip() for ele in cols]
                data.append([ele for ele in cols if ele])
        return uniqueAndSortInput(selects, types)


# Function which get only unique strings and sorts them by natural order
def uniqueAndSortInput(selects, part):
    semester = set()
    for selection in selects:
        if part == "values":
            if selection.attrs != {}:
                for val in selection.attrs.values():
                    try:
                        int(val)
                    except ValueError:
                        continue
                    semester.add(int(val))
        if part == "content":
            try:
                int(selection.contents[0])
                semester.add(selection.contents[0])
            except (IndexError, TypeError, ValueError):
                continue
        if part == "course":
            return selects
    return sorted(semester)


# Function which humanities dependencies
def cutDependencies(dependencies):
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
def getCourseInfo(course_number, index, total_number):
    temp_course = Course()
    types = get_info_from_graduate(course_number)
    temp_course.add_prerequisites(types[0])
    temp_course.add_linked(types[1])
    temp_course.add_identical(types[2])
    temp_course.add_overlapping(types[3])
    temp_course.add_inclusive(types[4])
    temp_course.add_including(types[5])
    temp_course.set_points(types[6])
    temp_course.set_name(types[7])
    temp_course.set_number(types[8])
    print('index: ' + str(index) + 'out of ' + str(total_number) + ' course:' + str(course_number))
    return temp_course


# Function which updates the Courses data baseS
def updateDb():
    initDB()
    course_numbers = sorted(getNumberOfCoursesList())
    index = 0
    course_numbers_length = len(course_numbers)
    for number in course_numbers:
        course = getCourseInfo(number, index, course_numbers_length)
        index += 1
        dbAddCourse(course)


def getNumberOfCoursesList():
    semester_tag = "input"
    semester_attrs = {"type": "radio", "name": "SEM"}
    faculties_tag = "option"
    faculties_attrs = {}
    search_url = "https://ug3.technion.ac.il/rishum/search"
    semesters = getData(search_url, semester_tag, semester_attrs, "values")
    faculties = getData(search_url, faculties_tag, faculties_attrs, "values")
    packages = []
    # Getting all the sports courses because they don't belong to any faculty
    for semester in semesters:
        for package in sportPackages(semester):
            packages.append(package)
    for combination in product(semesters, faculties):
        packages.append(preparePackage(combination[0], combination[1]))
    course_numbers = set()
    for package in packages:
        for course in getCourses(search_url, package):
            course_numbers.add(course)
    return course_numbers


# Function which creates possible packages to prompt ug for sport course with the given semester
def sportPackages(semester):
    package = preparePackage(semester, "")
    package2 = preparePackage(semester, "")
    package2["CNM"] = "חינוך גופני"
    package["CNM"] = "ספורט"
    return [package, package2]


# Function which initializes an courses data-base, does nothing if the db exists
def initDB():
    db = sqlite3.connect("./db/courses.db")
    curs = db.cursor()
    # prerequisites, linked, identical, overlapping, inclusive, including, points
    curs.execute(
        "CREATE TABLE IF NOT EXISTS courses(course_name STR,"
        "course_number TEXT PRIMARY KEY,"
        "points REAL,"
        "prerequisites BIT,"
        "linked BIT,"
        "identical BIT,"
        "overlapping BIT,"
        "inclusive BIT,"
        "including BIT)"
    )
    curs.close()
    db.close()


# Function which adds a course to the data base
def dbAddCourse(course):
    db = sqlite3.connect("./db/courses.db")
    curs = db.cursor()
    curs.execute("REPLACE INTO courses VALUES(?, ?, ?, ?, ?, ?, ?,?,?)", course.to_list())
    db.commit()
    curs.close()
    db.close()


# Function which converts data-base entry into course instance
def convertDbEnryToCourse(entry):
    temp_course = Course()
    temp_course.set_name(entry[0])
    temp_course.set_number(entry[1])
    temp_course.set_points(entry[2])
    temp_course.add_prerequisites(pickle.loads(entry[3]))
    temp_course.add_linked(pickle.loads(entry[4]))
    temp_course.add_identical(pickle.loads(entry[5]))
    temp_course.add_overlapping(pickle.loads(entry[6]))
    temp_course.add_inclusive(pickle.loads(entry[7]))
    temp_course.add_including(pickle.loads(entry[8]))
    return temp_course


# Function which search for the given course_number in the data-base
# return an course instance if found oneS
def findCourseInDB(course_number):
    db = sqlite3.connect("./db/courses.db")
    curs = db.cursor()
    course_number_tup = (course_number,)
    course = curs.execute(
        "SELECT * FROM courses WHERE  course_number=?", course_number_tup
    )
    result = course.fetchone()
    if not result:
        curs.close()
        db.close()
        return "הקורס לא נמצא במערכת, נסה שנית."
    else:
        curs.close()
        db.close()
        return convertDbEnryToCourse(result)


# Function which creates a list of all the courses in the data base
# return courses list
def loadCourseNameNumberPairs():
    db = sqlite3.connect("db/courses.db")
    curs = db.cursor()
    courses = curs.execute("SELECT * FROM courses ORDER BY course_number")
    dropdown = list()
    for course in courses:
        dropdown.append(str(course[1]) + " - " + course[0])
    curs.close()
    db.close()
    return dropdown


def main():
    updateDb()


if __name__ == "__main__":
    main()
