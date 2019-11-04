from flask import Flask, render_template, request, session
from flask_session import Session


app = Flask(__name__)

app.config["SESSION_PERMANENT"] = False
app.config["SESSION_TYPE"] = "filesystem"
Session(app)


@app.route("/")
def index():
    user = request.headers.get('User-Agent')
    linux_user = "Linux" in user
    return render_template("tab_template.html")

# @app.route("/course_info", methods=["GET", "POST"])
# def course_info():
#     if request.method == "GET":
#         return "There are no shortcuts!"
#     else:
#         course_number = request.form.get("course_number")
#         return render_template(f"course.html", course_info=course_to_html(course_number))

if __name__ == '__main__':
    app.run()
