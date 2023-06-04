import type { ComponentType, SvelteComponentTyped } from 'svelte/internal';
import {
    flexRender as flexRenderOrig,
} from '@tanstack/svelte-table';

// Workaround for https://github.com/TanStack/table/issues/4778
export const flexRender = <P extends Record<string, any>, C = any>(
    component: C,
    props: P
): ComponentType<SvelteComponentTyped> =>
    flexRenderOrig(component, props) as ComponentType<SvelteComponentTyped>;

// Generate a type for show
export class Show {
    public id: number = 0;
    public name: string = "";
    public start_date: Date;
    public location?: string;
    public entry_deadline?: Date;

    constructor(data: object) {
        Object.assign(this, data);
        let maybe_start_date = Object.getOwnPropertyDescriptor(data, "start_date");
        this.start_date = new Date(typeof maybe_start_date?.value === 'string' ? maybe_start_date.value : "");
        let maybe_entry_deadline = Object.getOwnPropertyDescriptor(data, "entry_deadline");
        this.entry_deadline = typeof maybe_entry_deadline?.value === 'string' ? new Date(maybe_entry_deadline.value) : undefined;
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

export class Rider {
    public id: number = 0;
    public name: string = "";
    public email: string = "";
    public membership_date?: Date;
    public birthday?: Date;
    public phone?: string;
    public address?: string;
    public person_responsible?: string;

    constructor(data: object) {
        Object.assign(this, data);

        let maybe_membership = Object.getOwnPropertyDescriptor(data, "membership_date");
        this.membership_date = typeof maybe_membership?.value === 'string' ? new Date(maybe_membership.value) : undefined;

        let maybe_birthday = Object.getOwnPropertyDescriptor(data, "birthday");
        this.birthday = typeof maybe_birthday?.value === 'string' ? new Date(maybe_birthday.value) : undefined;
    }
}
