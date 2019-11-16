import pickle
import sqlite3
import json
import bs4
from itertools import product
from datetime import date

import requests
from bs4 import BeautifulSoup

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
def getCourseInfo(course_number, semester):
    url = (
        "https://ug3.technion.ac.il/rishum/course/"
        + str(course_number)
        + "/"
        + str(semester)
    )
    tag = "div"
    attrs = {"class": "property"}
    types = "course"
    properties = getData(url, tag, attrs, types)
    strip = str.maketrans({"\n": None, "\r": None, "\t": None, "\xa0": " "})
    and_trans = str.maketrans({"ו": None, "-": "&"})
    or_trans = str.maketrans({"א": "|", "-": None})
    temp_course = Course()
    for prop in properties:
        sibling = prop.next_sibling.next_sibling.text.translate(strip)
        if "שם מקצוע" in prop.text:
            temp_course.set_name(sibling)
        if "מספר מקצוע" in prop.text:
            temp_course.set_number(sibling.strip())
        if "נקודות" in prop.text:
            temp_course.set_points(sibling.strip())
        if "מקצועות קדם" in prop.text:
            temp_course.add_dependencies(
                cutDependencies(sibling.translate(and_trans).translate(or_trans))
            )
        if "מקצועות צמודים" in prop.text:
            temp_course.add_parallel(sibling.split())
        if ":מקצועות ללא זיכוי נוסף" in prop.text:
            temp_course.add_similarities(sibling.split())
        if "מקצועות ללא זיכוי נוסף (מוכלים)" in prop.text:
            temp_course.add_inclusive(sibling.split())
    if temp_course.points == 0:
        temp_course.set_points(get_points_from_gradute(course_number))
    return temp_course


def get_points_from_gradute(course_number):
    url = "https://www.graduate.technion.ac.il/Subjects.Heb/?Sub=" + str(course_number)
    tag = "td"
    attrs = {"valign": "top"}
    data = []
    with requests.Session() as session:
        get = session.post(url)
        soup = BeautifulSoup(get.content, features="html5lib")
        if "valign" in attrs.keys():
            table = soup.find("table", attrs={"id": "points"})
            table_body = table.find("tbody")
            rows = table_body.find_all("tr")
            for row in rows:
                cols = row.find_all("td")
                cols = [ele.text.strip() for ele in cols]
                data.append([ele for ele in cols if ele])
    found_points = False
    for table in data:
        for row in table:
            if found_points:
                try:
                    return float(row)
                except (ValueError):
                    return 0
            if "זיכוי" in row:
                found_points = True
    return 0


# Function which updates the Courses data baseS
def updateDb(
    MainWindow, value=None, progress_bar_ui=None, stop_flag=None, stand_alone_flag=False
):
    initDB()
    if not stand_alone_flag:
        if MainWindow.english_ui:
            progress_bar_ui.label.setText("Collecting information:")
        else:
            progress_bar_ui.label.setText("אוסף מידע:")
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
    if not stand_alone_flag:
        if MainWindow.english_ui:
            progress_bar_ui.label.setText("(1/2) Collecting course numbers")
        else:
            progress_bar_ui.label.setText("(1/2) אוסף מספרי קורסים")
    for package in packages:
        for course in getCourses(search_url, package):
            if not stand_alone_flag:
                if MainWindow.progressBar:
                    progress_bar_ui.progressBar.setValue(
                        (len(course_numbers) / 600) % 6
                    )
            course_numbers.add(course)
            if not stand_alone_flag and stop_flag[0]:
                return
    counter = 0
    if not stand_alone_flag:
        if MainWindow.english_ui:
            progress_bar_ui.label.setText("(2/2) Updating courses:")
        else:
            progress_bar_ui.label.setText("(2/2) מעדכן קורסים")
    for course_number in sorted(course_numbers):
            if not stand_alone_flag and stop_flag[0]:
                return
            counter += 1
            if not stand_alone_flag:
                if MainWindow.progressBar:
                    value[0] = 5 + (counter / len(course_numbers)) * 95
                    progress_bar_ui.progressBar.setValue(value[0])
            dbAddCourse(getCourseInfo(course_number, semesters[len(semesters) - 1]))
    with open("settings.json", "r+") as write_file:
        data_json = json.load(write_file)
        data_json["updated"] = date.today().strftime("%B %d, %Y")
        write_file.seek(0)
        json.dump(data_json, write_file, indent=4)
        write_file.truncate()


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
    curs.execute(
        "CREATE TABLE IF NOT EXISTS courses(course_name STR,"
        "course_number TEXT PRIMARY KEY,"
        "points REAL,"
        "dependencies BIT,"
        "parallel BIT,"
        "similarities BIT,"
        "inclusive BIT)"
    )
    curs.close()
    db.close()


# Function which adds a course to the data base
def dbAddCourse(course):
    db = sqlite3.connect("./db/courses.db")
    curs = db.cursor()
    curs.execute("REPLACE INTO courses VALUES(?, ?, ?, ?, ?, ?, ?)", course.to_list())
    db.commit()
    curs.close()
    db.close()


# Function which converts data-base entry into course instance
def convertDbEnryToCourse(entry):
    temp_course = Course()
    temp_course.set_name(entry[0])
    temp_course.set_number(entry[1])
    temp_course.set_points(entry[2])
    temp_course.add_dependencies(pickle.loads(entry[3]))
    temp_course.add_parallel(pickle.loads(entry[4]))
    temp_course.add_similarities(pickle.loads(entry[5]))
    temp_course.add_inclusive(pickle.loads(entry[6]))
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
    updateDb(None,stand_alone_flag=True)

if __name__ == "__main__":
    main()