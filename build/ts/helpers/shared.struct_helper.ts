import {Value} from "../pb/shared.struct_pb.js";

type AllowedValue = null | number | string | boolean | Array<AllowedValue> | object;

export function constructValue(value: AllowedValue): Value {
    if (value === null) {
        return {kind: {oneofKind: "nullValue", nullValue: 0}};
    }
    if (typeof value === "number") {
        return {kind: {oneofKind: "numberValue", numberValue: value}};
    }
    if (typeof value === "string") {
        return {kind: {oneofKind: "stringValue", stringValue: value}};
    }
    if (typeof value === "boolean") {
        return {kind: {oneofKind: "boolValue", boolValue: value}};
    }
    if (Array.isArray(value)) {
        const listValues = value.map(v => constructValue(v));
        return {
            kind: {
                oneofKind: "listValue",
                listValue: {values: listValues}
            }
        };
    }
    if (typeof value === "object") {
        const structFields: {[key: string]: Value} = {};
        for (const [k, v] of Object.entries(value)) {
            structFields[k] = constructValue(v);
        }
        return {
            kind: {
                oneofKind: "structValue",
                structValue: {fields: structFields}
            }
        };
    }
    throw new Error(`Unsupported value type: ${typeof value}`);
}