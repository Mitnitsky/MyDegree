import pickle


# Technion courses class
# contains information about an course
# e.g:
#   name   = "מערכות הפעלה"
#   number = "234123"
#   dependencies = {[234218]} courses which are pre-requisite
#   parallel = {[234118]} courses which are either pre-requisite or must be taken together with the course
#   similarities = {[]} courses which are similar to this course
#   inclusive = {[]} courses which are included in this course and covered by it
class Course:
    def __init__(self):
        self.name = ""
        self.number = ""
        self.points = 0
        self.dependencies = list()
        self.parallel = set()
        self.similarities = set()
        self.inclusive = set()
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

    def add_inclusive(self, courses):
        self.inclusive.update(courses)

    def add_dependencies(self, courses):
        stripped_courses = []
        strip = str.maketrans({"\n": None, "\r": None, "\t": None, "\xa0": " ", " ": None})
        for course in courses:
            courses_inner = []
            for c in course:
                courses_inner.append(c.translate(strip))
            stripped_courses.append(courses_inner)
        self.dependencies.extend(stripped_courses)

    def add_similarities(self, courses):
        self.similarities.update(courses)

    def add_parallel(self, courses):
        self.parallel.update(courses)

    # Prepare the course data into a list to insert into DB (some is getting serialized)
    def to_list(self):
        return [
            self.name, self.number, self.points,
            pickle.dumps(self.dependencies),
            pickle.dumps(self.parallel),
            pickle.dumps(self.similarities),
            pickle.dumps(self.inclusive)
        ]

    def reprDependencies(self, english=False):
        if len(self.dependencies) > 0:
            result = []
            if english:
                humanify = str.maketrans({",": " and", "'": None})
            else:
                humanify = str.maketrans({",": " ו-", "'": None})
            separator = ""
            for depend in self.dependencies:
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
                        + ("Pre-requisites: {} \n".format(self.reprDependencies(english=True)) if len(
                self.dependencies) > 0 else "") \
                        + ("Parallel courses: {} \n".format(self.repOtherData(self.parallel)) if len(
                self.parallel) > 0 else "") \
                        + ("Similar courses: {} \n".format(self.repOtherData(self.similarities)) if len(
                self.similarities) > 0 else "") \
                        + ("Inclusive courses: {} \n".format(self.repOtherData(self.inclusive)) if len(
                self.inclusive) > 0 else "")
        else:
            represent = "שם הקורס: {} \n".format(self.name) \
                        + "מספר קורס: {} \n".format(self.number) \
                        + ("מס' נקודות: {} \n".format(self.points) if self.points > 0 else "") \
                        + ("מקצועות קדם: {} \n".format(self.reprDependencies()) if len(self.dependencies) > 0 else "") \
                        + ("מקצועות צמודים: {} \n".format(self.repOtherData(self.parallel)) if len(
                self.parallel) > 0 else "") \
                        + ("מקצועות ללא זיכוי נוסף: {} \n".format(self.repOtherData(self.similarities)) if len(
                self.similarities) > 0 else "") \
                        + ("מקצועות ללא זיכוי נוסף (מוכלים): {} \n".format(self.repOtherData(self.inclusive)) if len(
                self.inclusive) > 0 else "")
        return represent
