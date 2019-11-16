import sqlite3  
import pickle 
import json

from course import Course

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

def main():
    conn = sqlite3.connect('courses.db')
    c = conn.cursor()
    courses = list()
    for row in c.execute('SELECT * FROM courses'):
        course = vars(convertDbEnryToCourse(row))
        course['similarities'] = list(course['similarities'])
        course['parallel'] = list(course['parallel'])
        course['inclusive'] = list(course['inclusive'])
        courses.append(course)
    for course in courses:
        if len(course['dependencies']) > 0:
            newDependenciesList = list()
            counter = 0
            for dep_list in course['dependencies']:
                newDependenciesList.append(list())
                for dep in dep_list:
                    for result in c.execute('SELECT course_name FROM courses where course_number=?', (dep,)):
                        if len(result) > 0:
                            newDependenciesList[counter].append(dep + ': ' + result[0])
                if len(newDependenciesList[counter]) > 0:
                    counter += 1
                else:
                    newDependenciesList.remove(newDependenciesList[counter]) 
            course['dependencies'] = newDependenciesList
        if len(course['similarities']) > 0:
            newSimilaritiesList = list()
            for similarities in course['similarities']:
                for result in c.execute('SELECT course_name FROM courses where course_number=?', (similarities,)):
                    if len(result) > 0:
                        newSimilaritiesList.append(dep + ': ' + result[0])
            course['similarities'] = newSimilaritiesList
        if len(course['parallel']) > 0:
            newParallelList = list()
            for parallel in course['parallel']:
                for result in c.execute('SELECT course_name FROM courses where course_number=?', (parallel,)):
                    if len(result) > 0:
                        newParallelList.append(dep + ': ' + result[0])
            course['parallel'] = newParallelList
        if len(course['inclusive']) > 0:
            newInclusiveList = list()
            for inclusive in course['inclusive']:
                for result in c.execute('SELECT course_name FROM courses where course_number=?', (inclusive,)):
                    if len(result) > 0:
                        newInclusiveList.append(dep + ' - ' + result[0])
            course['inclusive'] = newInclusiveList
        course['full_name'] = course['number'] + ' - ' + course['name']
        course.pop('english', None)
    with open('courses.json', 'w') as courses_file:
        courses_file.write(json.dumps({"courses": courses},indent=4,  ensure_ascii=False))

if __name__ == "__main__":
    main()