export const default_course_types = [
    "חובה",
    "פטור",
    "מל\"ג",
    "בחירה חופשית",
    "רשימה א'",
    "רשימה ב'"
];

export function create_course_type(type_name) {
    return {name: type_name, points_left: 0, points_required: 0}
}
