import time

import requests
from bs4 import BeautifulSoup, NavigableString
import re

from contextlib import closing
from dataclasses import dataclass

from selenium import webdriver
from selenium.common.exceptions import NoSuchElementException
from selenium.webdriver.common.by import By


UG_URL = "https://students.technion.ac.il/local/technionsearch/course/"
TECHNION_SEARCH_URL = "https://students.technion.ac.il/local/technionsearch"


@dataclass
class Course:
    name: str
    number: str
    # info@ unused fields
    faculty: str
    academic_level: str
    given_this_semester: str


def get_all_courses_from_page(driver) -> list[Course]:
    courses = []
    courses_elements = driver.find_elements(By.XPATH,
                                            "//tr[contains(@id, 'courses_results-table') and not(contains(@class,'emptyrow'))]")
    for course in courses_elements:
        number, name = re.split(" *- *", course.find_element(By.CLASS_NAME, "c1").text, 1)
        faculty = course.find_element(By.CLASS_NAME, "c2").text or ''
        academic_level = course.find_element(By.CLASS_NAME, "c3").text or ''
        given_this_semester = course.find_element(By.CLASS_NAME, "c4").text or ''
        courses.append(Course(name, number, faculty, academic_level, given_this_semester))
    return courses


def get_all_courses_numbers():
    courses = []
    with closing(webdriver.Firefox()) as driver:
        driver.get(TECHNION_SEARCH_URL)
        semesters = driver.find_elements(By.XPATH, "//*[contains(@id, 'id_semesterscheckboxgroup')]")
        for semester in semesters:
            try:
                if not semester.is_selected():
                    semester.click()
            except Exception as _:
                pass
        more_filters_link = driver.find_element(By.XPATH, "//*[contains(@role, 'button')]")
        more_filters_link.click()
        while True:
            try:
                down_arrow = driver.find_element(By.XPATH, "//*[contains(@id, 'form_autocomplete_downarrow')]")
                down_arrow.click()
                break
            except NoSuchElementException as _:
                pass
        time.sleep(5)
        while True:
            try:
                faculties = driver.find_elements(By.XPATH, "//li[contains(@aria-selected, 'false')]")
                if len(faculties) == 0:  # skipping dummy text first element
                    break
                for faculty in faculties[0:]:
                    if 'ניתן לבחור' not in faculty.accessible_name or not len(faculty.accessible_name) == 0:
                        print(faculty.accessible_name)
                        faculty.click()
                        time.sleep(0.2)
                        down_arrow.click()
                        time.sleep(0.2)
                        break
            except Exception as _:
                pass
        search_button = driver.find_element(By.XPATH, "//input[contains(@name, 'submitbutton')]")
        search_button.click()
        while True:
            try:
                courses.extend(get_all_courses_from_page(driver))
                next_page = driver.find_elements(By.XPATH, "//a[contains(@class, 'page-link')]")
                if "»" in next_page[-1].text:
                    next_page[-1].click()
                else:
                    break
            except NoSuchElementException as _:
                break
            except Exception as e:
                print(e)
    result = []
    for course in courses:
        result.append(course.number)
    return result


def get_points_from_grad(soup):
    paragraphs = soup.find_all("p", attrs={"class": "card-text"})
    for paragraph in paragraphs:
        if 'נקודות אקדמיות' in paragraph.text:
            trimmed = paragraph.text.replace("\n", "").strip().split('•')
            for val in trimmed:
                if 'נקודות אקדמיות' in val:
                    result = val.strip().replace('נקודות אקדמיות', '').strip()
                    return float(result)
    return 0.0


def get_name_and_number_from_students(soup):
    title = soup.find("title")
    name = title.text
    return name


def get_info_from_students(course_number):
    url = UG_URL + str(course_number)
    result = {
        'מקצועות קדם': [[]],
        'מקצועות צמודים': [],
        'מקצועות זהים': [],
        'מקצועות ללא זיכוי נוסף': [],
        'מקצועות ללא זיכוי נוסף (מוכלים)': [],
        'מקצועות ללא זיכוי נוסף (מכילים)': []
    }
    with requests.Session() as session:
        get = session.post(url)
        if get.status_code != 200:
            print(f"Warning: Couldn't fetch course {course_number}!")
        soup = BeautifulSoup(get.content, features="html5lib")
        points = get_points_from_grad(soup)
        course_name_number = re.split(' *- *', get_name_and_number_from_students(soup).replace('\n', ''))
        course_name = ''
        if len(course_name_number) > 2:
            for i in range(0, len(course_name_number)):
                if i == 0:
                    continue
                else:
                    course_name += " " + course_name_number[i].strip()
        else:
            course_number, course_name = course_name_number
        categories_div = soup.find("h3", attrs={"class": "card-title"}).find_next(
            "div")  # first one should be 'מידע כללי'
        h5_s = []
        for child in categories_div.children:
            if type(child) != NavigableString and child.name == 'h5':
                h5_s.append(child)
        for cat in h5_s:
            p = cat.next_sibling.find_next('p')
            for child in p.children:
                text = child.text.strip()
                if cat.text == 'מקצועות קדם':
                    if text == '(':
                        continue
                    elif text == ')':
                        continue
                    elif text == ") או (":
                        result['מקצועות קדם'].append([])
                    elif text == "'או-'":
                        result['מקצועות קדם'].append([])
                    elif text == 'או-':
                        continue
                    elif text == 'ו-':
                        continue
                    elif text == '':
                        continue
                    else:
                        if '-' in text:
                            course_name_inn = ": ".join(re.split(' *- *', text, 1))
                            if len(course_name_inn) != 0:
                                result[cat.text][len(result[cat.text]) - 1].append(course_name_inn)
                else:
                    if cat.text == 'מקצועות מכילים':
                        if '-' in text:
                            course_name_inn = ": ".join(re.split(' *- *', text, 1))
                            if len(course_name_inn) != 0:
                                result['מקצועות ללא זיכוי נוסף (מכילים)'].append(course_name_inn)
                    else:
                        if '-' in text:
                            course_name_inn = ": ".join(re.split(' *- *', text, 1))
                            if len(course_name_inn) != 0:
                                result[cat.text].append(course_name_inn)
    return result['מקצועות קדם'], result['מקצועות צמודים'], result['מקצועות זהים'], result['מקצועות ללא זיכוי נוסף'], \
           result['מקצועות ללא זיכוי נוסף (מוכלים)'], result[
               'מקצועות ללא זיכוי נוסף (מכילים)'], points, course_name, course_number
