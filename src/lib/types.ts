// Generate a type for show
export class Show {
    public id: number = 0;
    public name: string = "";
    public start_date: string = "";
    public location?: string;
    public entry_deadline?: string;

    constructor(data: Partial<Show>) {
        Object.assign(this, data);
    }
};

export class Division {
    public id: number = 0;
    public name: string = "";

    constructor(data: Partial<Division>) {
        Object.assign(this, data);
    }
}

export class Class {
    public id: number = 0;
    public name: string = "";
    public division_id: number = 0;

    constructor(data: Partial<Class>) {
        Object.assign(this, data);
    }
}
