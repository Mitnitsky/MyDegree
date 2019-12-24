export const course_types = {
    MUST: 0,
    LIST_A: 1,
    LIST_B: 2,
    HUMANISTIC: 3,
    FREE_CHOICE: 4,
    EXEMPTION: 5
};
export const default_course_types = [
    "חובה",
    "מל\"ג",
    "בחירה חופשית",
    "פטור",
    "רשימה א'",
    "רשימה ב'"
];

export function create_course_type(type_name) {
    window.console.log(type_name)
    return {name: type_name, points_left: 0, points_required: 0}
}
