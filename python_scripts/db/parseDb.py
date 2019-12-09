import sqlite3
import pickle
import json

from course import Course


def convert_db_entry_to_course(entry):
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


def main():
    conn = sqlite3.connect('courses_backup.db')
    cursor = conn.cursor()
    courses = list()
    types = ['linked', 'identical', 'overlapping', 'inclusive', 'including']
    for row in cursor.execute('SELECT * FROM courses'):
        course = vars(convert_db_entry_to_course(row))
        for course_type in types:
            course[course_type] = list(course[course_type])
        courses.append(course)
    for course in courses:
        # extract_prerequisites(cursor, course)
        # for course_type in types:
        #     extract_name_for_type(cursor, course, course_type)
        course['full_name'] = course['number'] + ': ' + course['name']
        course.pop('english', None)
    with open('courses.json', 'w') as courses_file:
        courses_file.write(json.dumps({"courses": courses}, indent=4, ensure_ascii=False))


def extract_prerequisites(coursor, course):
    if len(course['prerequisites']) > 0:
        new_prerequisites_list = list()
        counter = 0
        for dep_list in course['prerequisites']:
            new_prerequisites_list.append(list())
            for dep in dep_list:
                for result in coursor.execute('SELECT course_name FROM courses where course_number=?', (dep,)):
                    if len(result) > 0:
                        new_prerequisites_list[counter].append(dep + ': ' + result[0])
            if len(new_prerequisites_list[counter]) > 0:
                counter += 1
            else:
                new_prerequisites_list.remove(new_prerequisites_list[counter])
        course['prerequisites'] = new_prerequisites_list


def extract_name_for_type(cursor, course, type_name='identical'):
    if len(course[type_name]) > 0:
        new_identical_list = list()
        for identical in course[type_name]:
            for result in cursor.execute('SELECT course_name FROM courses where course_number=?', (identical,)):
                if len(result) > 0:
                    new_identical_list.append(identical + ': ' + result[0])
        course[type_name] = new_identical_list


if __name__ == "__main__":
    main()
