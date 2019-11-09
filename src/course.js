export default class Course {
    constructor() {
        this.existsInDB = false;
        this.name = 'OS';
        this.number = '234123';
        this.points = '3';
        this.grade = '100';
        this.type = '0';
    }

    jsonify() {
        return JSON.stringify(this);
    }
    isEmpty() {
        return this.name == '' && this.number == '';
    }
    clear(){
        this.existsInDB = false;
        this.name = '';
        this.number = '';
        this.points = '';
        this.grade = '';
        this.type = '0';
    }
}
