export const default_course_types_obj = [
    {name:    "חובה", points_left:0, points_required: 0},
    {name:    "פטור", points_left:0, points_required: 0},
    {name:    "מל\"ג", points_left:0, points_required: 0},
    {name:    "בחירה חופשית", points_left:0, points_required: 0},
    {name:    "רשימה א'", points_left:0, points_required: 0},
    {name:    "רשימה ב'", points_left:0, points_required: 0}
];

export function create_course_type(type_name) {
    return {name: type_name, points_left: 0, points_required: 0}
}
