import pickle
from course import Course
from contextlib import closing


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


# Function which initializes courses database, does nothing if the db already exists
def init_db(db):
    with closing(db.cursor()) as cursor:
        # prerequisites, linked, identical, overlapping, inclusive, including, points
        cursor.execute(
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


# Function which adds a course to the database
def db_add_course(db, course):
    with closing(db.cursor()) as cursor:
        cursor.execute("REPLACE INTO courses VALUES(?, ?, ?, ?, ?, ?, ?,?,?)", course.to_list())
        db.commit()


# Function which search for the given course_number in the database
# return n course instance if found one
def find_course_in_db(course_number, db):
    with closing(db.cursor()) as cursor:
        course_number_tup = (course_number,)
        course = cursor.execute(
            "SELECT * FROM courses WHERE  course_number=?", course_number_tup
        )
        result = course.fetchone()
        if result:
            return convert_db_entry_to_course(result)
        else:
            return "הקורס לא נמצא במערכת, נסה שנית."


# Function which creates a list of all the courses in the database
# return courses list
def load_course_name_number_pairs(db):
    with closing(db.cursor()) as cursor:
        courses = cursor.execute("SELECT * FROM courses ORDER BY course_number")
        return list([str(course[1]) + " - " + course[0] for course in courses])


def extract_all_courses_with_types_from_db(db, types):
    courses = []
    with closing(db.cursor()) as cursor:
        for row in cursor.execute('SELECT * FROM courses'):
            course = vars(convert_db_entry_to_course(row))
            for course_type in types:
                course[course_type] = list(course[course_type])
            courses.append(course)
    return courses
