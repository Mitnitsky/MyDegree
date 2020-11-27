export const default_course_types_obj = [
  {name: "חובה", total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0},
  {name: "פטור", total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0},
  {name: 'מל"ג', total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0},
  {name: "בחירה חופשית", total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0},
  {name: "רשימה א'", total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0},
  {name: "רשימה ב'", total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0}
];

export function create_course_type(type_name) {
  return {name: type_name, total_points: 0, points_left: 0, points_required: 0, points_done: 0, average: 0};
}
