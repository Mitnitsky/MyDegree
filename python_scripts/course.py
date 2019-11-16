import pickle

vars
# Technion courses class
# contains information about an course
# e.g:
#   name   = "מערכות הפעלה"
#   number = "234123"
#   dependencies = {[234218]} courses which are pre-requisite
#   parallel = {[234118]} courses which are either pre-requisite or must be taken together with the course
#   similarities = {[]} courses which are similar to this course
#   inclusive = {[]} courses which are included in this course and covered by it
#   included = {[]} courses which are including this course and covering it
class Course:
    def __init__(self):

        self.full_name = ""
        self.name = ""
        self.number = ""
        self.points = 0
        self.prerequisites = list()
        self.linked = set()
        self.identical = set()
        self.overlapping = set()
        self.inclusive = set()
        self.including = set()
        self.english = False

    def set_name(self, name):
        self.name = name

    def set_number(self, number):
        self.number = number

    def set_points(self, points):
        try:
            self.points = float(points)
        except ValueError:
            self.points = 0

    # prerequisites, linked, identical, overlapping, inclusive, including
    def add_inclusive(self, courses):
        self.inclusive.update(courses)

    def add_identical(self, courses):
        self.identical.update(courses)

    def add_including(self, courses):
        self.including.update(courses)

    def add_prerequisites(self, courses):
        self.prerequisites.extend(courses)

    def add_overlapping(self, courses):
        self.overlapping.update(courses)

    def add_linked(self, courses):
        self.linked.update(courses)

    # Prepare the course data into a list to insert into DB (some is getting serialized)
    def to_list(self):
        return [
            self.name, self.number, self.points,
            pickle.dumps(self.prerequisites),
            pickle.dumps(self.linked),
            pickle.dumps(self.identical),
            pickle.dumps(self.overlapping),
            pickle.dumps(self.inclusive),
            pickle.dumps(self.including)
        ]

    def reprDependencies(self, english=False):
        if len(self.prerequisites) > 0:
            result = []
            if english:
                humanify = str.maketrans({",": " and", "'": None})
            else:
                humanify = str.maketrans({",": " ו-", "'": None})
            separator = ""
            for depend in self.prerequisites:
                result.append(separator)
                result.append(str(depend).translate(humanify))
                if english:
                    separator = " or "
                else:
                    separator = " או- "
            return ''.join(result)
        else:
            return ""

    def repOtherData(self, data):
        if len(data) > 0:
            result = ""
            humanify = str.maketrans({"{": None, "}": None, "'": None})
            return str(data).translate(humanify)
        else:
            return ""

    def __repr__(self):
        if self.english:
            represent = "Course name: {} \n".format(self.name) \
                        + "Course number: {} \n".format(self.number) \
                        + ("Points: {} \n".format(self.points) if self.points > 0 else "") \
                        + ("Pre-requisites: {} \n".format(self.reprDependencies(english=True)) if len(self.prerequisites) > 0 else "") \
                        + ("Parallel courses: {} \n".format(self.repOtherData(self.linked)) if len(self.linked) > 0 else "") \
                        + ("Similar courses: {} \n".format(self.repOtherData(self.identical)) if len(self.identical) > 0 else "") \
                        + ("Inclusive courses: {} \n".format(self.repOtherData(self.inclusive)) if len(self.inclusive) > 0 else "") \
                        + ("Including courses: {} \n".format(self.repOtherData(self.including)) if len(self.including) > 0 else "")
        else:
            represent = "שם הקורס: {} \n".format(self.name) \
                        + "מספר קורס: {} \n".format(self.number) \
                        + ("מס' נקודות: {} \n".format(self.points) if self.points > 0 else "") \
                        + ("מקצועות קדם: {} \n".format(self.reprDependencies()) if len(self.prerequisites) > 0 else "") \
                        + ("מקצועות צמודים: {} \n".format(self.repOtherData(self.linked)) if len(
                self.linked) > 0 else "") \
                        + ("מקצועות ללא זיכוי נוסף: {} \n".format(self.repOtherData(self.identical)) if len(
                self.identical) > 0 else "") \
                        + ("מקצועות ללא זיכוי נוסף (מוכלים): {} \n".format(self.repOtherData(self.inclusive)) if len(
                self.inclusive) > 0 else "") \
                        + ("מקצועות ללא זיכוי נוסף (כלולים): {} \n".format(self.repOtherData(self.including)) if len(
                self.including) > 0 else "")
        return represent
