function arrayRemove(semesters, index, value) {
    semesters[index.toString()] = semesters[index.toString()].filter((ele) => ele !== value);
    return semesters;
}

export function parseGraduateInformation(grades_copy) {
    grades_copy = grades_copy.split('\n');
    let lines = [[]];
    let index = 0;
    let found_first_sem = false;
    let english_exemption = false;
    let semesters = {};
    for (let line of grades_copy) {
        if (found_first_sem === false) {
            if (line.includes('אנגלית') && line.includes('פטור')) {
                english_exemption = true;
            }
            if (line.includes('קיץ') || line.includes('חורף') || line.includes('אביב')) {
                found_first_sem = true
            }
        } else {
            if (line.includes('קיץ') || line.includes('חורף') || line.includes('אביב')) {
                index += 1;
                lines.push([]);
                continue
            }
            if (!line.includes('ציון') && !line.includes('ממוצע') && !line.includes('הצלחות') && !line.includes('לא השלים')) {
                lines[index].push(line)
            }
        }
    }
    index = 1;
    for (let semester of lines) {
        let courses = [];
        for (let line of semester) {
            let course = {};
            if (line.length > 1 && line.trim().length > 1) {
                let parts = line.split('\t');
                course['grade'] = parts[0].replace('-', '').replace('*', '').replace('לא השלים', '');
                course['points'] = parts[1];
                let course_full_name = parts[2].split(' ');
                course['name'] = course_full_name.slice(0, -1).join(' ');
                course['number'] = course_full_name[course_full_name.length - 1];
                for (let i = 1; i < index; i++) {
                    let to_remove_list = [];
                    for (let cour of semesters[i.toString()]) {
                        if (!cour['name'].includes('ספורט') && !cour['name'].includes('חינוך') && !cour['name'].includes('נבחרות')) {
                            if (cour['name'] === course['name'] && course['grade'] !== '' && ((cour['grade'] !== '' && cour['grade'] !== 'לא השלים') || (course['grade'] === '' || course['grade'] === 'לא השלים'))) {
                                to_remove_list.push(cour)
                            }
                        }
                    }
                    for (let rem of to_remove_list) {
                        semesters = arrayRemove(semesters, i, rem)
                    }
                }
                for (let already_added of courses) {
                    if (already_added['name'] === course['name']) {
                        already_added['grade'] = course['grade'];
                        course = null;
                    }
                }
                if (course !== null) {
                    courses.push(course)
                }
            }
        }
        semesters[index.toString()] = courses;
        index += 1
    }
    return {"semesters": semesters, "exemption": english_exemption};
}