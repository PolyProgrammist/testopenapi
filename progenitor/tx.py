import json
import re
import itertools

def reconstructAllOfOneOf(schema):
    # return
    print('reconstructing allOf to oneOf. only for testing with progenitor')
    all_of = schema["allOf"]
    
    # Check if all elements in allOf have oneOf
    one_of_lists = []
    for item in all_of:
        if "oneOf" in item:
            one_of_lists.append(item["oneOf"])
        else:
            # If there's an element in allOf that doesn't have oneOf,
            # we'll treat it as a oneOf with a single element
            one_of_lists.append([item])
    
    # Generate all combinations of the elements from all oneOf arrays
    combinations = list(itertools.product(*one_of_lists))
    
    # Create a new oneOf array with allOf entries for each combination
    new_one_of = []
    for combo in combinations:
        combined = {
            "allOf": list(combo)
        }
        new_one_of.append(combined)
    
    # Replace the allOf with oneOf in the schema
    schema.pop("allOf")
    schema["oneOf"] = new_one_of

def iterate_nested_json_for_loop(json_obj):
    if isinstance(json_obj, dict):
        if '$ref' in json_obj and json_obj['$ref'] == "#/components/schemas/Rational32SchemaProvider" and 'default' in json_obj and not isinstance(json_obj['default'], dict):
            json_obj['default'] = {
                'denom': json_obj['default'][0],
                'numer': json_obj['default'][1]
            }
            json_obj['allOf'] = [
                {
                    "$ref": "#/components/schemas/Rational32SchemaProvider"
                }
            ]
            del json_obj['$ref']
        if 'const' in json_obj:
            t = json_obj['const']
            del json_obj['const']
            json_obj['enum'] = [t]
        for key, value in json_obj.items():
            iterate_nested_json_for_loop(value)
        if 'allOf' in json_obj:
            oneOfs = 0
            for item in json_obj['allOf']:
                if 'oneOf' in item:
                    oneOfs += 1
            if oneOfs >= 2:
                reconstructAllOfOneOf(json_obj)
    if isinstance(json_obj, list):
        for item in json_obj:
            iterate_nested_json_for_loop(item)

filename = '../testokplain/transaction.json'
f = open(filename)
filedata = f.read()
f.close()

filedata = filedata.replace('"type": "null"', '"type": "object"')

f = open(filename, 'w')
f.write(filedata)
f.close()


f = open(filename, 'r')
spec = json.load(f)
f.close()

iterate_nested_json_for_loop(spec)

spec['components']['schemas']['BlockHeaderView']['properties']['approvals']['items'] = {
    "allOf": [
        {
            "$ref": "#/components/schemas/Signature"
        }
    ],
    "nullable": True
}

spec['components']['schemas']['RpcLightClientNextBlockResponse']['properties']['approvals_after_next']['items'] = {
    "allOf": [
        {
            "$ref": "#/components/schemas/Signature"
        }
    ],
    "nullable": True
}

spec['components']['schemas']['CauseRpcErrorKind'] = {
    "anyOf": [
        {
            "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
        },
        {},
        {}
    ]
}



f = open(filename, 'w')
json.dump(spec, f, indent=4)
f.close()


filename = './keeper/src/lib.rs'
f = open(filename, 'r')
filedata = f.read()
f.close()

newfiledata = re.sub('"{}/\w*', '"{}/', filedata)

f = open(filename, 'w')
f.write(newfiledata)
f.close()


# cd testokplain && cargo run > transaction.json && cd ../progenitor && python3 tx.py && cargo progenitor -i ../testokplain/transaction.json -o keeper -n keeper -v 0.1.0 && python3 tx.py && cd user && cargo run && cd ../..