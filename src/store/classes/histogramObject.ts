export interface Option {
  students: string;
  passFail: string;
  passPercent: string;
  min: string;
  max: string;
  average: string;
  median: string;
  semester_name: string;
  semester_number: string;
  entry_name: string;
  staff: string;
}

export interface OptionsObject {
  value: Option[];
  text: string;
}

export interface HistogramObject {
  label: string;
  semester_number: string;
  options: OptionsObject[];
}
