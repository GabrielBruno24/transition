/* tslint:disable */

/**
 * This file has been automatically generated by the [capnpc-ts utility](https://github.com/jdiaz5513/capnp-ts).
 */

import * as capnp from 'capnp-ts';
import { ObjectSize as __O, Struct as __S } from 'capnp-ts';
import { Place } from './place.capnp';
export const _capnpFileId = 'c432eb24a9ca5905';
export class PlaceCollection extends __S {
    static readonly _capnp = { displayName: 'PlaceCollection', id: 'd409cc267e3d980f', size: new __O(0, 1) };
    static _Places: capnp.ListCtor<Place>;
    adoptPlaces(value: capnp.Orphan<capnp.List<Place>>): void {
        __S.adopt(value, __S.getPointer(0, this));
    }
    disownPlaces(): capnp.Orphan<capnp.List<Place>> {
        return __S.disown(this.getPlaces());
    }
    getPlaces(): capnp.List<Place> {
        return __S.getList(0, PlaceCollection._Places, this);
    }
    hasPlaces(): boolean {
        return !__S.isNull(__S.getPointer(0, this));
    }
    initPlaces(length: number): capnp.List<Place> {
        return __S.initList(0, PlaceCollection._Places, length, this);
    }
    setPlaces(value: capnp.List<Place>): void {
        __S.copyFrom(value, __S.getPointer(0, this));
    }
    toString(): string {
        return 'PlaceCollection_' + super.toString();
    }
}
PlaceCollection._Places = capnp.CompositeList(Place);
