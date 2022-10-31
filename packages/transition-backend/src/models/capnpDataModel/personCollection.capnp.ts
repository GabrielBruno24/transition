/* tslint:disable */

/**
 * This file has been automatically generated by the [capnpc-ts utility](https://github.com/jdiaz5513/capnp-ts).
 */

import * as capnp from 'capnp-ts';
import { ObjectSize as __O, Struct as __S } from 'capnp-ts';
import { Person } from './person.capnp';
export const _capnpFileId = 'f9b31db38bb1e2ed';
export class PersonCollection extends __S {
    static readonly _capnp = { displayName: 'PersonCollection', id: 'd974bc2836320626', size: new __O(0, 1) };
    static _Persons: capnp.ListCtor<Person>;
    adoptPersons(value: capnp.Orphan<capnp.List<Person>>): void {
        __S.adopt(value, __S.getPointer(0, this));
    }
    disownPersons(): capnp.Orphan<capnp.List<Person>> {
        return __S.disown(this.getPersons());
    }
    getPersons(): capnp.List<Person> {
        return __S.getList(0, PersonCollection._Persons, this);
    }
    hasPersons(): boolean {
        return !__S.isNull(__S.getPointer(0, this));
    }
    initPersons(length: number): capnp.List<Person> {
        return __S.initList(0, PersonCollection._Persons, length, this);
    }
    setPersons(value: capnp.List<Person>): void {
        __S.copyFrom(value, __S.getPointer(0, this));
    }
    toString(): string {
        return 'PersonCollection_' + super.toString();
    }
}
PersonCollection._Persons = capnp.CompositeList(Person);
