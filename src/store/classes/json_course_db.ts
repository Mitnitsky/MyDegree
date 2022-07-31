export interface JsonCourse {
  full_name: string
  name: string
  number: string
  points: number
  prerequisites: string[][]
  linked: string[]
  identical: string[]
  overlapping: string[]
  inclusive: string[]
  including: string[]
  followed_by: string[]
}

export interface JsonCourseDB {
  version: number | null
  courses: JsonCourse[]
}
