import requests
from bs4 import BeautifulSoup

TYPES = [
    'מקצועות קדם',
    'מקצועות צמודים',
    'מקצועות זהים',
    'מקצועות ללא זיכוי נוסף (מוכלים)',
    'מקצועות ללא זיכוי נוסף (מכילים)',
    'מקצועות ללא זיכוי נוסף',
    'מקצועות מכילים'
]

UG_URL = "https://www.graduate.technion.ac.il/Subjects.Heb/?Sub="

def safe_remove_from_list(arr, element):
    if element in arr:
        arr.remove(element)


def extract_course_number(arr):
    for val in arr:
        try:
            int(val)
            return val
        except ValueError:
            continue


def extract_course_name(arr):
    for val in arr:
        try:
            int(val)
        except ValueError:
            return val


def get_points_from_grad(soup):
    data = []
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

def get_name_and_number_from_graduate(soup):
    data = []
    span = soup.find("span", attrs={"dir": "rtl"})
    name = span.text.strip()
    return name


def get_info_from_graduate(course_number):
    url = UG_URL + str(course_number)
    data = []
    categories = {}
    prerequisites = [[]]
    linked = []
    identical = []
    overlapping = []
    inclusive = []
    including = []
    points = 0
    with requests.Session() as session:
        get = session.post(url)
        soup = BeautifulSoup(get.content, features="html5lib")
        points = get_points_from_grad(soup)
        course_name_number = get_name_and_number_from_graduate(soup).replace('\n', '').split(' - ')
        course_name = ''
        if len(course_name_number) > 2:
            for i in range(0,len(course_name_number)):
                if i == 0:
                    course_number = course_name_number[i]
                else:
                    course_name += course_name_number[i]
        else:
            course_number, course_name = course_name_number
        table = soup.find("table", attrs={"class": "tab0"})
        if table is not None:
            table_body = table.find("tbody")
            rows = table_body.find_all("tr")
            for row in rows:
                cols = row.find_all("td")
                cols = [ele.text.strip() for ele in cols]
                data.append([ele for ele in cols if ele])
            new_category = ''
            for table in data:
                for item in table:
                    if item in TYPES:
                        categories[item] = []
                        new_category = item
                if new_category != '':
                    categories[new_category].append(table)
            try:
                for i in categories['מקצועות ללא זיכוי נוסף (מכילים)']:
                    categories['מקצועות מכילים'].append(i)
                categories['מקצועות ללא זיכוי נוסף (מכילים)'] = None
            except KeyError:
                pass
            for (name, value) in categories.items():
                if name == 'מקצועות קדם':
                    for line in value:
                        if len(line) > 0:
                            if 'או' in line:
                                prerequisites.append([])
                            copy_line = line.copy()
                            strip_arr_from_symbols(copy_line)
                            prerequisites[-1].append(
                                extract_course_number(copy_line) + ': ' + extract_course_name(copy_line))
                    continue
                elif name == 'מקצועות צמודים':
                    get_courses_of_type(linked, value, name)
                elif name == 'מקצועות זהים':
                    get_courses_of_type(identical, value, name)
                elif name == 'מקצועות ללא זיכוי נוסף':
                    get_courses_of_type(overlapping, value, name)
                elif name == 'מקצועות ללא זיכוי נוסף (מוכלים)':
                    get_courses_of_type(inclusive, value, name)
                elif name == 'מקצועות מכילים':
                    get_courses_of_type(including, value, name)
    return prerequisites, linked, identical, overlapping, inclusive, including, points, course_name , course_number


def get_courses_of_type(type_arr, type_elements, type_name):
    for line in type_elements:
        if len(line) > 0:
            copy_line = line.copy()
            strip_arr_from_symbols(copy_line)
            if type_name in copy_line:
                copy_line.remove(type_name)
            type_arr.append(extract_course_number(copy_line) + ': ' + extract_course_name(copy_line))


def strip_arr_from_symbols(copy_line):
    safe_remove_from_list(copy_line, ')')
    safe_remove_from_list(copy_line, '(')
    safe_remove_from_list(copy_line, 'או')
    safe_remove_from_list(copy_line, 'ו-')
    safe_remove_from_list(copy_line, '-')
    safe_remove_from_list(copy_line, 'ו')