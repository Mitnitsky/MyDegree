export const default_course_types = [
    "חובה",
    "מל\"ג",
    "בחירה חופשית",
    "פטור",
    "רשימה א'",
    "רשימה ב'"
];

export function create_course_type(type_name) {
    return {name: type_name, points_left: 0, points_required: 0}
}
