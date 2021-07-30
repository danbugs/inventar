export class IncomingThing {
    constructor(thing_name, user_name, category_id = '', thing_description) {
        this.thing_name = thing_name;
        this.user_name = user_name;
        this.category_id = category_id;
        this.thing_description = thing_description;
    }
}

export class LoginUser {
    constructor(user_name, user_pwd) {
        this.user_name = user_name;
        this.user_pwd = user_pwd;
    }
}

export class RegisterUser {
    constructor(user_email, user_name, user_pwd) {
        this.user_email = user_email;
        this.user_name = user_name;
        this.user_pwd = user_pwd;
    }
}

export class IncomingCategory {
    constructor(category_name, category_color, user_name) {
        this.category_name = category_name;
        this.category_color = category_color;
        this.user_name = user_name;
    }
}